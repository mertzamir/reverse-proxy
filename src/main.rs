mod api;

use api::handler;
use api::models::*;

use actix_web::{get, web, App, HttpServer};
use std::time::{SystemTime};
use std::sync::{Arc, RwLock};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let cache = Arc::new(RwLock::new(Cache::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(cache.clone()))
        .service(handler::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}