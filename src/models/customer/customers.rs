use crate::diesel::RunQueryDsl;
use crate::models::errors::error::ApiError;
use crate::schemas::table_schemas::customers;
use argon2::Config;
use diesel::{self, prelude::*, Insertable, QueryDsl, Queryable};
use diesel::{PgConnection, QueryResult};
use rand::Rng;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Insertable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "customers"]
pub struct Customer {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub customer_phone: String,
    pub customer_email: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Customer {
    pub fn build(conn: &PgConnection, mut customer: Customer) -> Result<Self, ApiError> {
        customer.hash_password()?;
        let customer = diesel::insert_into(customers::table)
            .values(customer)
            .get_result(conn)?;

        Ok(customer)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;
        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }

    pub fn create_many(conn: &PgConnection, customer: &[Customer]) -> QueryResult<usize> {
        diesel::insert_into(customers::table)
            .values(customer)
            .execute(conn)
    }

    pub fn list_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        customers::table.load::<Customer>(conn)
    }

    pub fn list_customer_by_id(conn: &PgConnection, customer_id: &Uuid) -> QueryResult<Self> {
        customers::table
            .filter(customers::id.eq(customer_id))
            .first(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::customer::customers::mocks::*;
    use diesel::connection::Connection;
    use diesel::query_dsl::RunQueryDsl;
    use diesel::result::Error;
    use factori;
    use sunday_appetizers::establish_connection;

    #[test]
    fn should_insert_customers_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let customers = vec![factori::create!(Customer), factori::create!(Customer)];
            Customer::create_many(&conn, &customers).unwrap();
            let customers_result = customers::table.load::<Customer>(&conn)?;
            assert_eq!(customers_result, customers);
            Ok(())
        });
    }

    #[test]
    fn should_list_all_customers_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let customers = vec![factori::create!(Customer), factori::create!(Customer)];
            Customer::create_many(&conn, &customers).unwrap();
            let customers_result = Customer::list_all(&conn).unwrap();
            assert_eq!(customers_result.len(), 2);
            Ok(())
        });
    }

    #[test]
    fn should_list_customers_by_id_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let customers = vec![factori::create!(Customer)];
            diesel::insert_into(customers::table)
                .values(&customers)
                .execute(&conn)?;
            let customer = Customer::list_customer_by_id(&conn, &customers[0].id)?;

            assert_eq!(customer, customers[0]);
            Ok(())
        });
    }
}

pub mod mocks {
    use super::*;
    use factori;

    factori::factori!(Customer, {
        default {
            id = Uuid::new_v4(),
            first_name = String::from("Lucas"),
            last_name = String::from("hage"),
            customer_phone = String::from("329898989"),
            customer_email = String::from("lrchaves@gmail.com"),
            login = String::from("hage9998"),
            password = String::from("123"),
        }
    });
}
