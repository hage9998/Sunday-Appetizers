use crate::schemas::table_schemas::{customer_orders, customer_orders_status};
use chrono::NaiveDateTime;
use diesel::{self, prelude::*, Insertable, Queryable};
use diesel::{PgConnection, QueryResult};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(DbEnum, Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum TypesStatus {
    Concluded,
    InPreparation,
    InDelivering,
    Accepted,
    Refused,
}

#[derive(
    Associations, Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "customer_orders"]
#[belongs_to(Customer, OrdersStatus foreign_key= "customer_id, order_status_code")]
pub struct Orders {
    id: i32,
    customer_id: Uuid,
    order_status_code: i32,
    date_order_placed: NaiveDateTime,
    date_order_paid: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq, Clone, Queryable, Identifiable, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "customer_orders_status"]
pub struct OrdersStatus {
    id: i32,
    status_name: TypesStatus,
    status_description: String,
}

impl OrdersStatus {
    pub fn create_many(conn: &PgConnection, order_status: &OrdersStatus) -> QueryResult<usize> {
        diesel::insert_into(customer_orders_status::table)
            .values(order_status)
            .execute(conn)
    }

    pub fn list_cstomer_order_status_by_id(
        conn: &PgConnection,
        order_status_id: &i32,
    ) -> QueryResult<Self> {
        customer_orders_status::table
            .filter(customer_orders_status::id.eq(order_status_id))
            .first(conn)
    }

    pub fn list_all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        customer_orders_status::table.load::<OrdersStatus>(conn)
    }
}

impl Orders {
    pub fn create(conn: &PgConnection, order: &Orders) -> QueryResult<usize> {
        diesel::insert_into(customer_orders::table)
            .values(order)
            .execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::customer::customers::mocks::*;
    use crate::models::customer::customers::Customer;
    use crate::models::customer::orders_status::mocks::*;
    use diesel::connection::Connection;
    use diesel::result::Error;
    use sunday_appetizers::establish_connection;

    #[test]
    fn should_insert_customer_order_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let orders_status = factori::create!(OrdersStatus);
            OrdersStatus::create_many(&conn, &orders_status).unwrap();
            let orders_status_result = customer_orders_status::table
                .first::<OrdersStatus>(&conn)
                .unwrap();
            assert_eq!(orders_status_result, orders_status);
            Ok(())
        });
    }

    #[test]
    fn should_list_all_customers_order_status_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let orders_status = factori::create!(OrdersStatus, id: 2);
            OrdersStatus::create_many(&conn, &orders_status).unwrap();
            let orders_status_result = OrdersStatus::list_all(&conn).unwrap();
            assert_eq!(orders_status_result.len(), 1);
            Ok(())
        });
    }

    #[test]
    fn should_insert_order_correctly() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {
            let order_status = factori::create!(OrdersStatus);
            OrdersStatus::create_many(&conn, &order_status).unwrap();
            let customers = vec![factori::create!(Customer), factori::create!(Customer)];
            Customer::create_many(&conn, &customers).unwrap();
            let order = factori::create!(Orders, customer_id:customers[0].id);
            Orders::create(&conn, &order).unwrap();
            let order_result = customer_orders::table.first::<Orders>(&conn)?;
            assert_eq!(order_result, order);
            Ok(())
        });
    }
}

pub mod mocks {
    use super::*;
    use factori;

    factori::factori!(OrdersStatus, {
        default {
            id = 1,
            status_description = String::from("Produto em rota de entrega"),
            status_name = TypesStatus::InDelivering,
        }
    });

    factori::factori!(Orders, {
        default {
            id = 1,
            customer_id = Uuid::new_v4(),
            order_status_code = 1,
            date_order_placed = NaiveDateTime::from_timestamp(1_000_000_000, 0),
            date_order_paid = NaiveDateTime::from_timestamp(1_000_000_000, 0),

        }
    });
}
