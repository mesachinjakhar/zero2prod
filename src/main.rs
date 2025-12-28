use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener =
        TcpListener::bind("127.0.0.1:8000").expect("Failed to bind address on  port 8000");
    let server = run(listener);
    server.await?.await
}
