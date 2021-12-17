use crate::schemas::table_schemas::adresses;
use diesel::{self, prelude::*, Insertable, Queryable};
use diesel::{PgConnection, QueryResult};

#[derive(Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "adresses"]
pub struct Address {
    id: i32,
    address_street: String,
    address_number: i32,
    address_district: String,
    zip_postcode: String,
    city_name: String,
    state_name: String,
    country_name: String,
    other_address_details: String,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::address::address_data::mocks::*;
    use diesel::connection::Connection;
    use diesel::result::Error;
    use factori;
    use pate_project::establish_connection;

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
}
