#[tokio::test]
async fn health_check_works() {
    spawn_app().await; // we need to await it because we need to wait untill the server is created and handover to tokio

    // we need to wait untill the spawn_app do it works, eg creating server and running it on tokio
    // after that only we should execute our next line code eg: line 9

    // generate http client to send request
    let client = reqwest::Client::new();

    let respone = client.get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("Failed to execute request");

    assert!(respone.status().is_success());
    assert_eq!(Some(4), respone.content_length());
}

async fn spawn_app() {
    let server = zero2prod::run().await.expect("failed to bind address");

    let _ = tokio::spawn(server);
}