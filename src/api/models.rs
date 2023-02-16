use serde::{Deserialize};
use reqwest;
use std::collections::HashMap;
use std::time::{SystemTime};

// Struct for parsing the origin server URL
#[derive(Debug, Deserialize)]
pub struct OriginServer {
    url: String,
}

impl OriginServer {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

// Cache Models and Implementations
#[derive(Debug)]
pub struct CachedResponse {
    headers: reqwest::header::HeaderMap,
    status: reqwest::StatusCode,
    body: Vec<u8>,
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

    pub fn put(&mut self, key: &str, cached_response: CachedResponse) {
        self.data.insert(key.to_string(), cached_response);
    }

    pub fn remove_expired_entries(&mut self) {
        let now = SystemTime::now();
        self.data.retain(|_, value| value.expiration > now);
    }
}

impl CachedResponse {
    pub fn new(
        headers: reqwest::header::HeaderMap, 
        status: reqwest::StatusCode,
        body: Vec<u8>,
        expiration: SystemTime
    ) -> Self {
        CachedResponse {
            headers,
            status,
            body,
            expiration
        }
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn get_headers(&self) -> reqwest::header::HeaderMap {
        self.headers.clone()
    }

    pub fn get_status(&self) -> reqwest::StatusCode {
        self.status
    }

    pub fn get_expiration(&self) -> SystemTime {
        self.expiration
    }
}

