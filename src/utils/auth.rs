use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::customer::customers::Customer;
use crate::models::errors::error::ApiError;
use crate::schemas::table_schemas::customers;
use actix_session::Session;
use diesel::prelude::*;

pub fn authenticate(conn: &PgConnection, login: &str, password: &str) -> Result<(), ApiError> {
    let user = customers::table
        .filter(customers::login.eq(&login))
        .first::<Customer>(conn)?;

    let pass_validation: bool = user.verify_password(password.as_bytes())?;
    if login != user.login || !pass_validation {
        Err(ApiError::new(
            401,
            "Login or password incorrect".to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn is_logged_in(session: Session, token: &str) -> Result<(), ApiError> {
    let valid_session = session.get::<i32>(token);

    match valid_session {
        Ok(Some(_)) => Ok(()),
        _ => Err(ApiError::new(401, String::from("Invalid credentials"))),
    }
}
