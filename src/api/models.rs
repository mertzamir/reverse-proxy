use serde::{Deserialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

const CACHE_TTL_SECONDS: u64 = 30;

// Struct for parsing the origin server URL
#[derive(Debug, Deserialize)]
pub struct OriginServer {
    pub url: String,
}

// Cache
#[derive(Debug)]
pub struct CachedResponse {
    response: String,
    expiration: SystemTime,
}

#[derive(Debug)]
pub struct Cache {
    data: HashMap<String, CachedResponse>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&CachedResponse> {
        self.data.get(key)
    }

    pub fn put(&mut self, key: String, response: String) {
        let expiration = SystemTime::now() + Duration::from_secs(CACHE_TTL_SECONDS);
        self.data.insert(key, CachedResponse { response, expiration });
    }

    pub fn remove_expired_entries(&mut self) {
        let now = SystemTime::now();
        self.data.retain(|_, value| value.expiration > now);
    }
}

impl CachedResponse {
    pub fn get_response(&self) -> String {
        self.response.clone()
    }

    pub fn get_expiration(&self) -> SystemTime {
        self.expiration
    }
}

