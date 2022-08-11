use actix_web::{get, App, HttpServer, Responder, middleware};

#[get("/up")]
async fn health_check() -> impl Responder {
    "Server is Up!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health_check)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
