use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::config::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read app config.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(TcpListener::bind(address).unwrap())?.await
}
