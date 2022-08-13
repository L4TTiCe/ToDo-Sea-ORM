mod api;
mod dao;
mod database;
mod errors;
mod lib;
mod model;

use crate::database::MongoDB;

use actix_web::{get, middleware, App, HttpServer, Responder};
use dotenv::dotenv;
use errors::Error;

#[macro_use]
extern crate log;

#[get("/up")]
async fn health_check() -> impl Responder {
    info!("GET /up Health Check");
    "Server is Up!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    pretty_env_logger::init();

    info!("VERBOSE REST responses SET");
    std::env::set_var("VERBOSE_REST", "1");

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .unwrap();
    info!("Listening on port {}", port);

    let db = MongoDB::init().await?;

    let db_data = actix_web::web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .service(health_check)
            .configure(api::task::attach_service)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
    .map_err(|err| {
        error!("{}", err);
        Error::ServerStartFailed(err)
    })
}
