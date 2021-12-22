use crate::models::customer::customer_data::Customer;

use actix_web::Error;
use actix_web::{web, HttpResponse};
use pate_project::establish_connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use deadpool_postgres::Pool;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("register").route(web::post().to(register_new_user)));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    first_name: String,
    last_name: String,
    customer_phone: String,
    customer_email: String,
    login: String,
    password: String,
}

pub async fn register_new_user(credentials: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let new_user = Customer {
        id: Uuid::new_v4(),
        first_name: credentials.first_name.clone(),
        last_name: credentials.last_name.clone(),
        customer_phone: credentials.customer_phone.clone(),
        customer_email: credentials.customer_email.clone(),
        login: credentials.login.clone(),
        password: credentials.password.clone(),
    };

    let conn = establish_connection();
    Customer::build(&conn, new_user)?;

    Ok(HttpResponse::Ok().body("User created successfully"))
}
