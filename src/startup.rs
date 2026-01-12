use crate::configuration::DatabaseSettings;
use crate::configuration::{self, Settings};
use crate::email_client::{self, EmailClient};
use crate::routes::{health_check, subscibe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");

        let email_client = EmailClient::new(
            configuration.email_client.base_url.to_owned(),
            sender_email,
            configuration.email_client.authorization_token.to_owned(),
            std::time::Duration::from_secs(2),
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listner = TcpListener::bind(address)?;
        let port = listner.local_addr().unwrap().port();
        let server = run(listner, connection_pool, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_untill_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    // New parameter!
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscibe))
            // Register the connection as part of the application state
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

pub async fn build(configuration: &Settings) -> Result<Server, std::io::Error> {
    let connection_pool = get_connection_pool(&configuration.database);

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");

    let email_client = EmailClient::new(
        configuration.email_client.base_url.to_owned(),
        sender_email,
        configuration.email_client.authorization_token.to_owned(),
        std::time::Duration::from_secs(2),
    );

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listner = TcpListener::bind(address)?;
    run(listner, connection_pool, email_client)
}
