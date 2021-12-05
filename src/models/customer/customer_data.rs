use crate::diesel::RunQueryDsl;
use crate::schemas::table_schemas::customers;
use diesel::QueryResult;
use pate_project::establish_connection;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "customers"]
struct Customer {
    id: Uuid,
    first_name: String,
    last_name: String,
    customer_phone: String,
    customer_email: String,
}

impl Customer {
    #[allow(dead_code)]
    pub fn insert_test() -> QueryResult<()> {
        let conn = establish_connection();
        let customer = Self {
            id: Uuid::new_v4(),
            first_name: String::from("Lucas"),
            last_name: String::from("hage"),
            customer_phone: String::from("329898989"),
            customer_email: String::from("lrchaves@gmail.com"),
        };

        diesel::insert_into(customers::table)
            .values(&customer)
            .execute(&conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_list_all_business_groups() {
        Customer::insert_test().unwrap();
    }
}
