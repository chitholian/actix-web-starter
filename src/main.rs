#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web_starter::start_server(("0.0.0.0", 8888)).await
}
