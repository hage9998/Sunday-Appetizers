use crate::models::customer::customers::Customer;
use crate::models::errors::error::ApiError;
use crate::routes::auth::SessionAuth;
use crate::utils::auth::is_logged_in;
use crate::utils::db_conn::Pool;

use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
// use diesel::PgConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerResponse {
    customers: Vec<Customer>,
    total: u32,
}

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/customers").route(web::get().to(get_all_customers)));
}

pub async fn get_all_customers(
    session: Session,
    credentials: web::Json<SessionAuth>,
    pool: web::Data<Pool>,
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
    use std::env;

    use crate::{
        configs,
        routes::{auth::{IndexResponse, login, Credentials}, customer::CustomerResponse},
        utils::test_route::test_service,
    };
    use actix_redis::RedisSession;
    use actix_web::{
        http::{header, StatusCode},
        test::{self, TestRequest},
        web, App, HttpResponse,
    };
    use bytes::Bytes;
    use serde_json::json;
    use sunday_appetizers::establish_connection;

    use crate::utils::db_conn::Pool as NewPool;
    use diesel::{
        r2d2::{ConnectionManager, Pool},
        PgConnection,
    };

    #[actix_rt::test]
    async fn should_list_all_customers_correctly2() {
  {          let request_body = Credentials{
                login: "lobo".to_string(),
                password: "123".to_string(),
            };

            let pool = NewPool::test_pool().await;
            // let s = RedisSession::new("127.0.0.1:6379", &[0;32]);
            let mut app = test::init_service(App::new().data(pool.clone()).route("/login", web::post().to(login))).await;

            let response = TestRequest::post()
                .uri("/login")
                .set_json(&request_body)
                .send_request(&mut app)
                .await;
            println!("Logged");
            

            //   let respo: IndexResponse = test::read_body_json(resp).await;
            assert_eq!(response.status(), StatusCode::OK);}
    }
}
