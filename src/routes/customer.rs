use crate::models::customer::customers::Customer;
use crate::models::errors::error::ApiError;
use crate::routes::auth::Credentials;
use crate::utils::db_conn::Pool;

use actix_session::Session;
use actix_web::{web, HttpResponse};
// use diesel::PgConnection;
// use sunday_appetizers::establish_connection;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test").route(web::get().to(test)));
}

// pub  fn test(session: actix_session::Session, db_pool: web::Data<Pool>,) -> HttpResponse {
pub async fn test(
    session: Session,
    credentials: web::Json<Credentials>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ApiError> {
    let valid_session = session.get::<String>(&credentials.login).unwrap();

    if let Some(_valid_session) = valid_session {
        let conn = pool.get_conn();

        let customers = Customer::list_all(&conn)?;
        println!("{:?}", &customers);

        Ok(HttpResponse::Ok().body("testando"))
    } else {
        Err(ApiError::new(401, String::from("Invalid credentials")))
    }
}

// async fn do_something(session: Session) -> Result<HttpResponse> {
//     let user_id: Option<String> = session.get::<String>("user_id").unwrap();
//     let counter: i32 = session
//         .get::<i32>("counter")
//         .unwrap_or(Some(0))
//         .map_or(1, |inner| inner + 1);
//     session.set("counter", counter)?;

//     Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
// }
