extern crate actix_web;
extern crate env_logger;
extern crate serde;

use std::env;

use dotenv::dotenv;

use crate::infraestructure::http::Server;

mod infraestructure;
mod domain;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port = (env::var("PORT").unwrap_or("8080".to_string()))
        .parse::<u16>()
        .unwrap();

    let server = Server::new(port);

    server.run().await
}
