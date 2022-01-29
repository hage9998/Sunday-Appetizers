use crate::configs;
use crate::NewPool;

use std::env;

use actix_redis::SameSite;
use actix_service::ServiceFactory;
use actix_session::{CookieSession, Session};
use actix_web::{web, App};

use diesel::{r2d2::ConnectionManager, r2d2::Pool, Connection, PgConnection};
use r2d2::CustomizeConnection;

#[derive(Debug)]
struct TestTransaction;

impl CustomizeConnection<PgConnection, diesel::r2d2::Error> for TestTransaction {
    fn on_acquire(
        &self,
        conn: &mut PgConnection,
    ) -> ::std::result::Result<(), ::diesel::r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

pub async fn test_service() -> (
    App<
        impl ServiceFactory<
            Request = actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
        actix_web::dev::Body,
    >,
    NewPool,
) {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(2)
        .connection_customizer(Box::new(TestTransaction))
        .build(manager)
        .expect("Failed to init pool");

    let pool = NewPool { pool };

    // TODO Try to understand if i can call auth inside a wrap or service (/) instead
    // or try ti get user credentios and pass like data, i dunno;
    let app = App::new()
        .data(pool.clone())
        .wrap(
            CookieSession::signed(&[0; 32])
                .path("/")
                .name("actix-test")
                .domain("localhost")
                .http_only(true)
                .same_site(SameSite::Lax)
                .max_age(100),
        )
        .service(web::resource("/").to(|ses: Session| async move {
            ses.set("counter", 100).unwrap();
            "test"
        }))
        .configure(configs);

    (app, pool)
}
