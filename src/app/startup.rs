use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub struct PokedexApp {
    pub server: Result<Server, std::io::Error>,
    pub port: u16,
}

impl PokedexApp {
    pub async fn new(binding_address: &str) -> Result<PokedexApp, std::io::Error> {
        let tcp_listener = TcpListener::bind(binding_address)?;
        let port = tcp_listener.local_addr().unwrap().port();

        let server = HttpServer::new(move || {
            App::new().route("/health_check", web::get().to(HttpResponse::Ok))
        })
        .listen(tcp_listener)
        .map(HttpServer::run);
        Ok(PokedexApp { server, port })
    }
}
