macro_rules! validate_type {
    ($name: ident, $ty:ty, $helper:expr) => {
        pub fn $name(v: String) -> Result<(), String> {
            match v.parse::<$ty>() {
                Ok(..) => Ok(()),
                Err(..) => Err($helper.to_owned()),
            }
        }
    };
}

validate_type!(validate_usize, usize, "should be unsigned integer");
