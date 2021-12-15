pub mod models;
pub mod routes;
pub mod schemas;
#[macro_use]
pub extern crate diesel;
use crate::routes::costumer::configure_service;
use actix_web::{web, App, HttpResponse, HttpServer};

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(configure_service))
        .route("/", web::get().to(|| HttpResponse::Ok().body("alou")));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8002")?
        .run()
        .await
}
