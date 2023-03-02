use actix_web::{App, get, HttpServer, Responder};
use actix_web::web::Json;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("0.0.0.0", 8888))?.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    Json(json!({
        "status": 0,
        "message": "It works!"
    }))
}
