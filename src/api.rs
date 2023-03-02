use actix_web::web;
use actix_web::web::ServiceConfig;

mod controller;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1").configure(configure_v1));
}

fn configure_v1(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/test").to(controller::test))
        .route("/users", web::get().to(controller::user::all));
}
