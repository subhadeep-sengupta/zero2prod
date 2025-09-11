use std::net::TcpListener;
use zero2prod::configuration::*;
use zero2prod::startup::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Server is live!");
    run(listener)?.await
}
