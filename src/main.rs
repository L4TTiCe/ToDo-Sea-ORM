use actix_web::{get, App, HttpServer, Responder, middleware};
use dotenv::dotenv;

#[macro_use]
extern crate log;

#[get("/up")]
async fn health_check() -> impl Responder {
    info!("GET /up Health Check");
    "Server is Up!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    pretty_env_logger::init();

    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "4000".to_string()).parse::<u16>().unwrap();
    info!("Listening on port {}", port);
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
