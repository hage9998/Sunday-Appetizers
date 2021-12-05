use crate::diesel::RunQueryDsl;
use crate::schemas::table_schemas::customers;
use diesel::{self, Insertable, Queryable};
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Queryable, Identifiable, Insertable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "customers"]
pub struct Customer {
    id: Uuid,
    first_name: String,
    last_name: String,
    customer_phone: String,
    customer_email: String,
}

impl Customer {
    #[allow(dead_code)]
    pub fn create_many(conn: &PgConnection, customer: &Vec<Customer>) -> QueryResult<()> {
        diesel::insert_into(customers::table)
            .values(customer)
            .execute(conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::customer::customer_data::mocks::*;
    use diesel::connection::Connection;
    use diesel::query_dsl::RunQueryDsl;
    use diesel::result::Error;
    use factori;
    use pate_project::establish_connection;

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
        }
    });
}
