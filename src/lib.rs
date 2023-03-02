use std::collections::HashMap;

use actix_web::{App, get, HttpServer, Responder, web};
use actix_web::web::Json;
use serde_json::{json, Value};

mod api;
mod model;

pub async fn start_server(addr: (&str, u16)) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::NormalizePath::default())
            .service(web::scope("/api").configure(api::configure))
            .service(index)
    }).bind(addr)?.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    Json(json!({
        "status": 0,
        "message": "It works!"
    }))
}

pub fn json_resp(status: u16, data: Option<Value>, message: Option<&str>) -> impl Responder {
    let mut json_obj: HashMap<&str, Value> = HashMap::new();
    json_obj.insert("status", Value::from(status));
    if let Some(dat) = message {
        json_obj.insert("message", Value::from(dat));
    }
    if let Some(dat) = data {
        json_obj.insert("data", dat);
    }
    Json(json_obj)
}
