use actix_web::{web, get, post, Responder, HttpResponse};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime};
use super::models::{OriginServer, Cache};

#[get("/")]
pub async fn index(origin_server: web::Query<OriginServer>, cache: web::Data<Arc<RwLock<Cache>>>) -> Result<String, actix_web::Error> {
    let key = origin_server.url.to_string();
    let mut cache = cache.write().unwrap();

    // Check if the response is already in the cache
    if let Some(cached_response) = cache.get(&key) {
        print!("url found in cache");
        if cached_response.get_expiration() > SystemTime::now() {
            return Ok(cached_response.get_response());
        }
    } 

    // Query origin server and forward the response
    print!("Querying the origin server");

    // Save the response in cache
    let orig_response = "Original response".to_string();
    let cached_resp ="cached original response".to_string();
    cache.put(key, cached_resp.clone());
    cache.remove_expired_entries();

    Ok(orig_response)
}