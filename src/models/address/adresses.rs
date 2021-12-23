use crate::schemas::table_schemas::{adresses, customer_adresses};
use diesel::{self, prelude::*, Insertable, Queryable};
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

#[derive(Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "adresses"]
pub struct Address {
    pub id: i32,
    pub address_street: String,
    pub address_number: i32,
    pub address_district: String,
    pub zip_postcode: String,
    pub city_name: String,
    pub state_name: String,
    pub country_name: String,
    pub other_address_details: String,
}

#[derive(
    Associations, Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize,
)]
#[table_name = "customer_adresses"]
#[belongs_to(Customer, Address foreign_key= "customer_id, address_id")]
pub struct CustomerAddress {
    id: i32,
    customer_id: Uuid,
    address_id: i32,
}

impl Address {
    pub fn create_many(conn: &PgConnection, address: &[Address]) -> QueryResult<usize> {
        diesel::insert_into(adresses::table)
            .values(address)
            .execute(conn)
    }

    pub fn list_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        adresses::table.load::<Address>(conn)
    }

    pub fn list_adresses_by_id(conn: &PgConnection, address_id: &i32) -> QueryResult<Self> {
        adresses::table
            .filter(adresses::id.eq(address_id))
            .first(conn)
    }

    pub fn list_customer_address_by_customer_id(
        conn: &PgConnection,
        customer_id: &Uuid,
    ) -> QueryResult<Vec<Self>> {
        customer_adresses::table
            .inner_join(adresses::table)
            .filter(customer_adresses::customer_id.eq(customer_id))
            .select(adresses::all_columns)
            .load::<Address>(conn)
    }
}

impl CustomerAddress {
    pub fn create(conn: &PgConnection, customer_address: &CustomerAddress) -> QueryResult<usize> {
        diesel::insert_into(customer_adresses::table)
            .values(customer_address)
            .execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::address::adresses::mocks::*;
    use crate::models::customer::customers::mocks::*;
    use crate::models::customer::customers::Customer;
    use diesel::connection::Connection;
    use diesel::result::Error;
    use factori;
    use sunday_appetizers::establish_connection;

    #[test]
    fn should_insert_adresses_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let adresses = vec![factori::create!(Address)];
            Address::create_many(&conn, &adresses).unwrap();
            let adresses_result = adresses::table.load::<Address>(&conn).unwrap();
            assert_eq!(adresses_result, adresses);
            Ok(())
        });
    }

    #[test]
    fn should_list_all_adresses_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let adresses = vec![factori::create!(Address), factori::create!(Address, id: 2)];
            Address::create_many(&conn, &adresses).unwrap();
            let adresses_result = Address::list_all(&conn).unwrap();
            assert_eq!(adresses_result.len(), 2);
            Ok(())
        });
    }

    #[test]
    fn should_list_adresses_by_id_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let adresses = vec![factori::create!(Address)];
            diesel::insert_into(adresses::table)
                .values(&adresses)
                .execute(&conn)?;
            let address = Address::list_adresses_by_id(&conn, &adresses[0].id)?;

            assert_eq!(address, adresses[0]);
            Ok(())
        });
    }

    #[test]
    fn should_insert_customer_adresses_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let adresses = vec![factori::create!(Address)];
            Address::create_many(&conn, &adresses).unwrap();
            let customers = vec![factori::create!(Customer), factori::create!(Customer)];
            Customer::create_many(&conn, &customers).unwrap();
            let customer_adresses = factori::create!(CustomerAddress, customer_id:customers[0].id, address_id:adresses[0].id);
            CustomerAddress::create(&conn, &customer_adresses).unwrap();
            let customer_adresses_result= customer_adresses::table.first::<CustomerAddress>(&conn)?;
            assert_eq!(customer_adresses_result, customer_adresses);
            Ok(())
        });
    }

    #[test]
    fn should_list_customer_adress_by_id_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let adresses = vec![factori::create!(Address)];
            Address::create_many(&conn, &adresses).unwrap();
            let customers = vec![factori::create!(Customer), factori::create!(Customer)];
            Customer::create_many(&conn, &customers).unwrap();
            let customer_adresses = factori::create!(CustomerAddress, customer_id:customers[0].id, address_id:adresses[0].id);
            CustomerAddress::create(&conn, &customer_adresses).unwrap();
            let customer_address  =
            Address::list_customer_address_by_customer_id(&conn, &customers[0].id)?;
            assert_eq!(customer_address, adresses);
            Ok(())
        });
    }
}

pub mod mocks {
    use super::*;
    use factori;

    factori::factori!(Address, {
        default {
            id = 1,
            address_street = String::from("Rua Mairinque"),
            address_number = 11,
            address_district = String::from("Zona Norte"),
            zip_postcode = String::from("02637050"),
            city_name = String::from("São Paulo"),
            state_name = String::from("SP"),
            country_name = String::from("Brazil"),
            other_address_details = String::from("Proximo ao posto"),
        }
    });

    factori::factori!(CustomerAddress, {
        default {
            customer_id = Uuid::new_v4(),
            address_id = 1,
            id = 1,
        }
    });
}
