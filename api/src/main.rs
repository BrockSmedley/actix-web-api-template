mod models;
mod routes;

#[macro_use]
extern crate log;
use actix_web::{middleware, App, HttpServer};

use routes::index;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,api=info");
    env_logger::init();
    info!("beep boop...");

    HttpServer::new(|| {
        App::new()
            .configure(index::config)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
