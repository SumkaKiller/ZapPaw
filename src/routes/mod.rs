use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    tracing::info!("Получен запрос на /health");
    HttpResponse::Ok().json(json!({"status": "залупа(все ок)"}))
}