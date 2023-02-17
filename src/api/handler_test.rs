
use actix_web::{test, web, App, http::StatusCode};

use std::sync::{Arc, RwLock};

use super::handler::index;
use super::models::{Cache};

#[actix_web::test]
async fn test_index_ok() {
    let cache = web::Data::new(Arc::new(RwLock::new(Cache::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(index))).await;

    let req = test::TestRequest::get().uri("/?url=https://blockstream.info/api/blocks/0").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_index_no_url() {
    let cache = web::Data::new(Arc::new(RwLock::new(Cache::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(index))).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_index_broken_url() {
    let cache = web::Data::new(Arc::new(RwLock::new(Cache::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(index))).await;

    // correct URL should've been: https://httpbin.org/get
    let req = test::TestRequest::get().uri("/?url=https://httpbin.org/ge").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_index_not_get_request() {
    let cache = web::Data::new(Arc::new(RwLock::new(Cache::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(index))).await;

    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
