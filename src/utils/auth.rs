use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::customer::customers::Customer;
use crate::models::errors::error::ApiError;
use crate::schemas::table_schemas::customers;
use diesel::prelude::*;
use uuid::Uuid;

pub fn authenticate(
    conn: &PgConnection,
    login: &str,
    password: &str,
    customer_id: &Uuid,
) -> Result<(), ApiError> {
    let user = customers::table
        .filter(customers::id.eq(&customer_id))
        .first::<Customer>(conn)?;

    let pass_validation: bool = user.verify_password(password.as_bytes())?;
    println!("{:?}", pass_validation);
    if login != user.login || !pass_validation {
        Err(ApiError::new(
            401,
            "Login or password incorrect".to_string(),
        ))
    } else {
        Ok(())
    }
}
