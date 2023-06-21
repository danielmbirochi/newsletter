use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener, conn: PgPool) -> Result<Server, std::io::Error> {
    
    let conn_pool = web::Data::new(conn);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/healthcheck", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscriptions::subscribe))
            .app_data(conn_pool.clone()) 
    })
    .listen(listener)?
    .run();

    Ok(server)
}
