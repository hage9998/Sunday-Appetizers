use crate::utils::auth::authenticate;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use actix_session::Session;
use actix_web::{web, HttpResponse, Result};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
    pub customer_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    customer_id: Uuid,
    status: String,
}

pub async fn login(credentials: web::Json<Credentials>, session: Session) -> Result<HttpResponse> {
    let login = credentials.login.clone();
    let password = credentials.password.clone();
    let conn = pate_project::establish_connection();
    let customer_id = credentials.customer_id;

    authenticate(&conn, &login, &password, &customer_id)?;
    session.set(&login, &customer_id)?;
    session.renew();

    Ok(HttpResponse::Ok().json(IndexResponse {
        customer_id,
        status: String::from("Logged in successfully"),
    }))
}

pub async fn logout(credentials: web::Json<Credentials>, session: Session) -> Result<HttpResponse> {
    let id: Option<String> = session.get(&credentials.login)?;
    if let Some(x) = id {
        session.purge();
        Ok(format!("Logged out: {}", x).into())
    } else {
        Ok("Could not log out anonymous user".into())
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
