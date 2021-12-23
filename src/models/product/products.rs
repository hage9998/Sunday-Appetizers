use crate::schemas::table_schemas::products;
use bigdecimal::BigDecimal;
use diesel::{self, prelude::*, Insertable, Queryable};
use diesel::{PgConnection, QueryResult};
use diesel_derive_enum::DbEnum;

/// Payer work experience.
#[derive(DbEnum, Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum TypesProd {
    Bebida,
    Salgados,
    Doce,
}

#[derive(Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "products"]
pub struct Product {
    id: i32,
    product_price: BigDecimal,
    product_name: String,
    product_type: TypesProd,
}

impl Product {
    #[allow(dead_code)]
    pub fn create_many(conn: &PgConnection, customer: &[Product]) -> QueryResult<usize> {
        diesel::insert_into(products::table)
            .values(customer)
            .execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::product::products::mocks::*;
    use diesel::connection::Connection;
    use diesel::result::Error;
    use factori;
    use sunday_appetizers::establish_connection;

    #[test]
    fn should_insert_customers_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let customers = vec![factori::create!(Product)];
            Product::create_many(&conn, &customers).unwrap();
            let customers_result = products::table.load::<Product>(&conn).unwrap();
            assert_eq!(customers_result, customers);
            Ok(())
        });
    }
}

pub mod mocks {
    use super::*;
    use factori;

    factori::factori!(Product, {
        default {
            id = 1,
            product_price = BigDecimal::from(20),
            product_name = String::from("Coxinha"),
            product_type = TypesProd::Salgados,
        }
    });
}
