use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::{Connection, PgConnection, PgPool, Executor};
use newsletter::config::{get_configuration, DatabaseSettings};
use newsletter::telemetry::{get_tracing_subscriber, init_tracing_subscriber};
use once_cell::sync::Lazy;

// Ensure that `tracing` stack is only initialized once using once_cell.
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_tracing_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_tracing_subscriber(subscriber);
    } else {
        let subscriber = get_tracing_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_tracing_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub server_addr: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn subscribe_returns_400_when_fields_are_present_but_invalid() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=jon%20jones&email=", "missing email"),
        ("name=&email=bones%40jones.com", "missing name"),
        ("name=chuck&email=chuck", "invalid email"),
        ("name=&email=", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &test_app.server_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valie_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=jon%20jones&email=bones%40jones.com";
    let response = client
        .post(&format!("{}/subscribe", &test_app.server_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "bones@jones.com");
    assert_eq!(saved.name, "jon jones");

}

#[tokio::test]
async fn should_return_400_for_missing_fields() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=jon%20jones", "missing email"),
        ("email=bones%40jones.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
        .post(&format!("{}/subscribe", &test_app.server_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("failed to execute request");

        assert_eq!(
            400, 
            response.status().as_u16(),
            "should validate missing form fields. Expected 400 - {}, got {}",
            error_message,
            response.status().as_u16()
        );
    }

}

// cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthcheck", &test_app.server_addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let mut cfg = get_configuration().expect("Failed to read app config.");
    cfg.database.database_name = uuid::Uuid::new_v4().to_string();
    let db_pool = configure_database(&cfg.database).await;

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let server_port = listener.local_addr().unwrap().port();
    let server = newsletter::startup::run(listener, db_pool.clone()).expect("Failed to bind address");
    let server_addr = format!("http://127.0.0.1:{}", server_port);
    let _ = tokio::spawn(server);

    TestApp{
        server_addr,
        db_pool
    }
}

pub async fn configure_database(cfg: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&cfg.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    conn
        .execute(format!(r#"CREATE DATABASE "{}";"#, cfg.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&cfg.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}