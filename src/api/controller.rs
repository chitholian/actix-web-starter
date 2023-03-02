use actix_web::Responder;
use actix_web::web::Json;
use serde_json::json;

pub async fn test() -> impl Responder {
    Json(json!({
        "status": 0,
        "message": "API is working."
    }))
}

pub mod user {
    use actix_web::Responder;
    use serde_json::json;

    use crate::json_resp;
    use crate::model::User;

    pub async fn all() -> impl Responder {
        let user_1 = User::new("atik", "helloPassword", "chitholian@gmail.com", "Atikur Rahman");

        json_resp(200, Some(json!(vec![user_1])), None)
    }
}
