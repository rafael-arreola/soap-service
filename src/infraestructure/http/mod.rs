use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::ServiceConfig;

mod marshal;
mod structs;
mod handlers;

pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Server running on port {}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(
                    web::JsonConfig::default()
                        .error_handler(marshal::json_handler)
                        .limit(1024 * 1024 * 25),
                )
                .app_data(web::QueryConfig::default().error_handler(marshal::query_handler))
                .service(web::scope("/api/v1").configure(load_routes))
                .default_service(web::to(HttpResponse::NotFound))
        }).bind(("0.0.0.0", self.port))?.run().await
    }
}

fn load_routes(cfg: &mut ServiceConfig) {
    cfg.service(handlers::process_json);
}
