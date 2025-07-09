use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bind to port 8000 (or any other port you prefer)
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}