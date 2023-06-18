use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer};
use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscriptions::subscribe))
            
    })
    .listen(listener)?
    .run();

    Ok(server)
}
