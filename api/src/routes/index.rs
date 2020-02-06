use super::order;
use crate::models::test_data::TestData;

use actix_web::{
    // get,
    web,
    web::Json,
    Responder,
};

// #[get("/test/{id}/{name}")]
async fn index(info: web::Path<(u64, String)>) -> impl Responder {
    Json(TestData {
        val: info.0,
        name: info.1.to_owned(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order").service(
            web::resource("")
                .route(web::get().to(order::get_orders))
                .route(web::post().to(order::create_order)),
        ),
    )
    .service(
        web::scope("/test").service(
            web::scope("/{id}/{name}").service(web::resource("").route(web::get().to(index))),
        ),
    );
}
