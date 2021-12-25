use std::env;

use crate::configs;
use crate::NewPool;
use actix_web::{http::header, test, web, App, HttpResponse};
use diesel::r2d2::Pool;
use diesel::{r2d2::ConnectionManager, PgConnection};
use rand::Rng;

pub async fn test_service() -> impl actix_web::dev::Service {
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let pool = NewPool { pool };

    let mut app = test::init_service(
        App::new()
            .service(
                web::resource("/index.html")
                    .route(web::post().to(|| async { HttpResponse::Ok().body("welcome!") })),
            )
            .configure(configs),
    )
    .await;
    app
}
