mod config;

use actix_web::{get, HttpResponse, HttpServer};

#[get("/api")]
async fn status() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

macro_rules! create_app {
    () => {
        actix_web::App::new().service(crate::status)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::default();
    config.init_logger();
    let address = config.address();
    tracing::info!("starting server on {}", address);
    HttpServer::new(|| create_app!())
        .bind(config.address())?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_http::Request;
    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::test;

    #[derive(Default)]
    pub struct TestServer;

    impl TestServer {
        pub async fn execute(&self, req: Request) -> ServiceResponse {
            let app = test::init_service(create_app!()).await;
            test::call_service(&app, req).await
        }
    }

    #[actix_rt::test]
    async fn status() {
        let req = test::TestRequest::get().uri("/api").to_request();
        let res = TestServer::default().execute(req).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
