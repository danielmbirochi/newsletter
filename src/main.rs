use actix_web::{web, App, HttpServer, Responder, HttpResponse};


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
