use crate::utils::auth::authenticate;
use crate::utils::db_conn::Pool;

use serde::{Deserialize, Serialize};

use actix_session::Session;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionAuth {
    pub session_token: String,
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    session_token: String,
    session_counter: i32,
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

    authenticate(&conn, &login, &password)?;

    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .map_or(1, |inner| inner + 1);

    let session_token: Uuid = Uuid::new_v4();

    session.set(&session_token.to_string(), &counter)?;
    session.renew();

    Ok(HttpResponse::Ok().json(IndexResponse {
        session_token: session_token.to_string(),
        session_counter: counter,
        status: String::from("Logged in successfully"),
    }))
}

pub async fn logout(credentials: web::Json<SessionAuth>, session: Session) -> Result<HttpResponse> {
    let id: Option<i32> = session.get(&credentials.session_token)?;
    if id.is_some() {
        session.purge();
        Ok(format!("Logged out: {}", credentials.login).into())
    } else {
        Ok("Could not log out anonymous user".into())
    }
}
