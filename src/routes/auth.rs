use crate::utils::auth::authenticate;
use crate::utils::db_conn::Pool;

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

pub async fn login(
    credentials: web::Json<Credentials>,
    session: Session,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let login = credentials.login.clone();
    let password = credentials.password.clone();
    let conn = pool.get_conn();
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
