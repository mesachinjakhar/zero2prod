use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await; // we need to await it because we need to wait untill the server is created and handover to tokio

    // we need to wait untill the spawn_app do it works, eg creating server and running it on tokio
    // after that only we should execute our next line code eg: line 9

    // generate http client to send request
    let client = reqwest::Client::new();

    let respone = client.get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request");

    assert!(respone.status().is_success());
    assert_eq!(Some(4), respone.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind address for test");

    // get assigned port from listener 
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).await.expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}