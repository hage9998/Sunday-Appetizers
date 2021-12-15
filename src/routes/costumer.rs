use actix_web::{web, HttpResponse};

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test").route(web::get().to(test)));
}

fn test() -> HttpResponse {
    use crate::models::customer::customer_data::{mocks::*, *};
    use pate_project::establish_connection;
    let conn = establish_connection();
    let customers = vec![factori::create!(Customer)];
    Customer::create_many(&conn, &customers).unwrap();
    HttpResponse::Ok().body("testando")
}
