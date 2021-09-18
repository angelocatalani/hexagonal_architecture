use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Context;
use tracing_actix_web::TracingLogger;

use crate::app::Settings;
use crate::cache::CacheService;
use crate::pokeapi::PokeapiService;
use crate::routes;
use crate::translated::TranslatedService;

pub struct PokedexApp {
    pub server: Result<Server, anyhow::Error>,
    pub port: u16,
}

impl PokedexApp {
    pub async fn new(settings: Settings) -> anyhow::Result<PokedexApp> {
        let tcp_listener = TcpListener::bind(settings.binding_address())?;

        // since configuration port can be zero (e.g during tests)
        // we need to take the port at run time
        let port = tcp_listener
            .local_addr()
            .context("Fail to extract port from binding url")?
            .port();
        let pokeapi_service = web::Data::new(
            PokeapiService::new(
                settings.pokeapi_service.url,
                settings.pokeapi_service.timeout_seconds,
            )
            .context("Failed to instantiate PokeapiService")?,
        );
        let translated_service = web::Data::new(
            TranslatedService::new(
                settings.translated_service.url,
                settings.translated_service.timeout_seconds,
            )
            .context("Failed to instantiate PokeapiService")?,
        );
        let cache_service = web::Data::new(
            CacheService::new(settings.cache_service.url.as_ref())
                .await
                .context("Failed to instantiate PokeapiService")?,
        );
        let server = HttpServer::new(move || {
            App::new()
                .route("/health_check", web::get().to(HttpResponse::Ok))
                .route("/pokemon/{name}", web::get().to(routes::pokemon))
                .route(
                    "/pokemon/translated/{name}",
                    web::get().to(routes::pokemon_translated),
                )
                .app_data(pokeapi_service.clone())
                .app_data(translated_service.clone())
                .app_data(cache_service.clone())
                .wrap(TracingLogger::default())
        })
        .listen(tcp_listener)
        .map_or_else(|error| Err(error.into()), |server| Ok(server.run()));
        Ok(PokedexApp { server, port })
    }
}
