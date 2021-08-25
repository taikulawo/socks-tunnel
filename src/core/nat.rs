use std::{net::SocketAddr, time::Duration};

use lru_time_cache::LruCache;

pub struct NetworkTranslator {
    /// Used by TCP/UDP
    fake_mapping: LruCache<(SocketAddr, SocketAddr), SocketAddr>,
    connections: LruCache<>
}

const LRU_TTL: u64 = 24 * 60* 60;

impl NetworkTranslator {
    pub fn new() -> NetworkTranslator {
        NetworkTranslator {
            fake_mapping: LruCache::with_expiry_duration(Duration::from_secs(LRU_TTL)),
        }
    }
}