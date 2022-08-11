use actix_web::{get, App, HttpServer, Responder, middleware};

#[macro_use]
extern crate log;

#[get("/up")]
async fn health_check() -> impl Responder {
    info!("GET /up Health Check");
    "Server is Up!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    
    pretty_env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_check)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
