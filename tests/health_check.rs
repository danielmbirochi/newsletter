use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use newsletter::config::get_configuration;

#[tokio::test]
async fn subscribe_returns_a_200_for_valie_form_data() {
    let app_address = spawn_app().await;
    let cfg = get_configuration().expect("Failed to read app config.");
    // The `Connection` trait must be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct
    let mut conn = PgConnection::connect(&cfg.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let body = "name=jon%20jones&email=bones%40jones.com";
    let response = client
        .post(&format!("{}/subscribe", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "bones@jones.com");
    assert_eq!(saved.name, "jon jones");

}

#[tokio::test]
async fn should_return_400_for_missing_fields() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=jon%20jones", "missing email"),
        ("email=bones%40jones.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
        .post(&format!("{}/subscribe", &app_address))
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
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthcheck", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
