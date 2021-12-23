pub mod config;
pub mod models;
pub mod routes;
pub mod schemas;
pub mod utils;

#[macro_use]
pub extern crate diesel;
extern crate unique_type_id;
extern crate unique_type_id_derive;

use std::env;

use crate::routes::auth::{login, logout};
use crate::routes::customer::configure_service;
use crate::routes::register::configure_service as configure_service_register;
use crate::utils::db_conn::Pool as NewPool;

use actix_redis::RedisSession;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use rand::Rng;
// use tokio_postgres::NoTls;

fn configs(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(configure_service))
        .service(web::scope("/new").configure(configure_service_register))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .route("/", web::get().to(|| HttpResponse::Ok().body("alou")));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let pool = NewPool { pool };

    HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new("127.0.0.1:6379", &private_key).ttl(3600))
            .data(pool.clone())
            .configure(configs)
    })
    .bind("127.0.0.1:8002")?
    .run()
    .await
}

// TO DO: Maybe will necessary to add this later.

// use actix_cors::Cors;
// use actix_session::CookieSession;

// .wrap(CookieSession::signed(&[0; 32]).secure(false))
// .wrap(
//     Cors::default()
//         .allow_any_method()
//         .allow_any_origin()
//         .allow_any_header()
//         .max_age(3600),
// )
