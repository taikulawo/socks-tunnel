use std:: {future:: {
        Future
    }, io, pin::Pin, task::{Context, Poll}, u8};

use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
pub struct Pipe<'a, R: ?Sized, W: ?Sized> {
    from: &'a mut R,
    to: &'a mut W,
    buf: Option<Vec<u8>>,
    is_read_done: bool,
    position: usize,
    capacity: usize,
}

macro_rules! try_poll {
    ($expr:expr) => {
        match $expr {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
            Poll::Ready(Ok(n)) => n
        }
    };
}

// ?Sized之后会作为引用存在，这样Copy就能在编译时确认大小
// 也就能函数中返回Copy这结构体
impl<R, W> Future for Pipe<'_, R, W> 
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin
{
    type Output = io::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let myself = &mut *self;
            if myself.buf.is_none() {
                myself.buf = Some(Vec::with_capacity(1024));
            }
            // 如果一个字段用Option包住，再借用里面的对象，二次 deref挺不方便的
            // 不过这样也行，还在我能控制的范围内 :)
            // 也可以不用Option，刚开始创建的时候new一个。
            // 上面判断了is_none，这里unwrap不会panic
            let mut buf = ReadBuf::new(&mut **myself.buf.as_mut().unwrap());
            try_poll!(Pin::new(&mut myself.from).poll_read(cx, &mut buf));
            let len = buf.filled().len();
            if len == 0 {
                self.is_read_done = true;
                return Poll::Ready(Ok(()));
            }else {
                // 重置index，下面开始写到 to
                self.position = 0;
                self.capacity = len;
            }

            while self.position < self.capacity {
                let myself = &mut *self;
                let buf = &**myself.buf.as_mut().unwrap();
                let n = try_poll!(Pin::new(&mut myself.to).poll_write(cx, &buf[myself.position .. myself.capacity]));
                if n == 0 {
                    return Poll::Ready(Err(io::Error::new(io::ErrorKind::WriteZero, "zero write!!")))
                }
                self.position += n;
            };
            if self.position == self.capacity && self.is_read_done {
                let myself = &mut *self;
                try_poll!(Pin::new(&mut myself.to).poll_flush(cx));
                return Poll::Ready(Ok(()))
            };
        }
    }
}

pub fn pipe<'a, R, W>(from: &'a mut R, to: &'a mut W) -> Pipe<'a, R, W>
where
    R: AsyncRead + Unpin + ?Sized,
    W: AsyncWrite + Unpin + ?Sized
{
    Pipe {
        from,
        to,
        buf:None,
        capacity: 0,
        position: 0,
        is_read_done: false,
    }
}