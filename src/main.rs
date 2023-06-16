use std::net::TcpListener;

use newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(TcpListener::bind("127.0.0.1:3000").unwrap())?.await
}
