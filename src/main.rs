use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::config::get_configuration;
use newsletter::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_tracing_subscriber("newsletter".into(), "info".into());
    init_tracing_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read app config.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(
        TcpListener::bind(address).unwrap(),
        connection_pool,
    )?.await
}
