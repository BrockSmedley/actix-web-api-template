use actix_web::{web::Json, Responder};

use crate::models::test_data::TestData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateOrderResponse {
    val: u64,
    name: String,
    awesome: bool,
}

pub async fn create_order(params: Json<TestData>) -> impl Responder {
    // "send to DB"
    let v = params.into_inner();
    Json(CreateOrderResponse {
        val: v.val,
        name: v.name,
        awesome: true,
    })
}

pub async fn get_orders() -> impl Responder {
    Json([TestData {
        val: 0,
        name: "info".to_owned(),
    }])
}
