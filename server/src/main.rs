mod config;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::default();
    config.init_logger();
    let address = config.address();
    tracing::info!("starting server on {}", address);
    HttpServer::new(|| App::new().service(index))
        .bind(config.address())?
        .run()
        .await
}
