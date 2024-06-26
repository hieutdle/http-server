use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);
    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
