use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::config;
use newsletter::telemetry;
use sqlx::PgPool;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_tracing_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    telemetry::init_tracing_subscriber(subscriber);

    let cfg = config::get_configuration().expect("Failed to read app config.");
    let db_conn_pool = PgPool::connect(&cfg.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let app_addr = format!("{}:{}", cfg.application.host, cfg.application.port);

    run(
        TcpListener::bind(app_addr).unwrap(),
        db_conn_pool,
    )?.await
}
