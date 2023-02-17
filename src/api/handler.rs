use actix_web::{web, error, get, Result, HttpResponse};
use reqwest;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use log::{debug, info};
use super::models::{OriginServer, Cache, CachedResponse};

const CACHE_TTL_SECONDS: u64 = 30;

#[get("/")]
pub async fn index(origin_server: web::Query<OriginServer>, cache: web::Data<Arc<RwLock<Cache>>>) -> Result<HttpResponse, actix_web::Error> {
    let key = origin_server.into_inner().get_url();
    let mut cache = cache.write().unwrap();
    info!("Origin server: {}", key);

    // Check if the response is already in the cache
    if let Some(cached_response) = cache.get(&key) {
        if cached_response.get_expiration() > SystemTime::now() {
            // Return the cached response
            debug!("URL found in cache.");
            let mut headers = HttpResponse::build(cached_response.get_status());
            for (header_name, header_value) in cached_response.get_headers().iter() {
                headers.insert_header((header_name, header_value));
            }
            return Ok(headers.body(cached_response.get_body()));
        }
    } 

    // Send request to the origin server
    debug!("Querying the origin server");
    let response = match reqwest::get(&key).await {
        Ok(resp) => resp,
        Err(e) => return Err(error::ErrorInternalServerError(format!("Error sending request: {}", e))),
    };

    // Extract HTTP headers, status code, and body from the original response
    let cloned_headers = response.headers().clone();
    let status = response.status();
    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Err(error::ErrorInternalServerError(format!("Error reading response body: {}", e))),
    };
    let body = body_bytes.to_vec();

    // Save the original response in cache
    let expiration = SystemTime::now() + Duration::from_secs(CACHE_TTL_SECONDS);
    let cached_response = CachedResponse::new(cloned_headers.clone(), status, body.clone(), expiration);
    cache.put(&key, cached_response);
    cache.remove_expired_entries();

    // Populate the HttpResponse with original metadata
    let mut headers = HttpResponse::build(status);
    for (header_name, header_value) in cloned_headers.iter() {
        headers.insert_header((header_name.clone(), header_value.clone()));
    }
    Ok(headers.body(body))
}
