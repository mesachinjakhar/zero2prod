use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Pong")
}

async fn subscibe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscibe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
