use crate::models::errors::error::ApiError;
use crate::utils::db_conn::Pool;
use crate::{models::customer::customers::Customer, utils::auth::is_logged_in};

use super::auth::SessionAuth;
use actix_session::Session;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerResponse {
    customers: Vec<Customer>,
    total: u32,
}

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/customers").route(web::get().to(get_all_customers)));
}

pub async fn get_all_customers(
    credentials: web::Json<SessionAuth>,
    pool: web::Data<Pool>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    is_logged_in(session, &credentials.session_token)?;

    let conn = pool.get_conn();
    let customers = Customer::list_all(&conn)?;

    Ok(HttpResponse::Ok().json(CustomerResponse {
        customers: customers.clone(),
        total: customers.len() as u32,
    }))
}

#[cfg(test)]
mod tests {

    use crate::{
        models::customer::customers::mocks::*,
        models::customer::customers::Customer,
        routes::{auth::SessionAuth, customer::CustomerResponse},
        utils::test_route::test_service,
    };
    use actix_web::{
        dev::Service,
        test::{self, read_body_json, TestRequest},
    };

    #[actix_rt::test]
    async fn should_get_all_customers_correctly() {
        {
            {
                let (app, pool) = test_service().await;
                let mut app = test::init_service(app).await;
                let request = test::TestRequest::get().to_request();
                let response = app.call(request).await.unwrap();
                let cookie = response
                    .response()
                    .cookies()
                    .find(|c| c.name() == "actix-test")
                    .unwrap()
                    .clone();

                let payload = SessionAuth {
                    session_token: "counter".to_string(),
                    login: "login".to_string(),
                };
                let customers_new = {
                    let conn = pool.get_conn();
                    let customers = vec![factori::create!(Customer, password: "".to_string())];
                    Customer::create_many(&conn, &customers).unwrap();
                    customers
                };

                let path = "/internal/customers";
                let req = TestRequest::get()
                    .uri(path)
                    .cookie(cookie)
                    .set_json(&payload)
                    .to_request();
                let res = test::call_service(&mut app, req).await;
                let body: CustomerResponse = read_body_json(res).await;
                let CustomerResponse { customers, total } = body;

                assert_eq!(total, 1);
                assert_eq!(customers, customers_new);
            }
        }
    }
}
