use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Context;
use tracing_actix_web::TracingLogger;

use crate::configuration::settings::Settings;
use crate::pokemon_bounded_context::adapter::out::{FuntranslationApi, PokeApi, RedisCache};
use crate::pokemon_bounded_context::adapter::route;
use crate::pokemon_bounded_context::port::service::{PokemonInfo, PokemonTranslator};

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

        let pokemon_info = web::Data::new(PokemonInfo::new(
            PokeApi::new(settings.poke_api.url, settings.poke_api.timeout_seconds)
                .context("Failed to instantiate `PokeApi` client")?,
        ));

        let pokemon_translator = web::Data::new(PokemonTranslator::new(
            FuntranslationApi::new(
                settings.funtranslation_api.url.clone(),
                settings.funtranslation_api.timeout_seconds,
            )?,
            FuntranslationApi::new(
                settings.funtranslation_api.url,
                settings.funtranslation_api.timeout_seconds,
            )?,
            RedisCache::new(settings.redis_cache.url.as_str()).await?,
            RedisCache::new(settings.redis_cache.url.as_str()).await?,
        ));

        let server = HttpServer::new(move || {
            App::new()
                .route("/health_check", web::get().to(HttpResponse::Ok))
                .route("/pokemon/{name}", web::get().to(route::pokemon))
                .route(
                    "/pokemon/translated/{name}",
                    web::get().to(route::pokemon_translated),
                )
                .app_data(pokemon_info.clone())
                .app_data(pokemon_translator.clone())
                .wrap(TracingLogger::default())
        })
        .listen(tcp_listener)
        .map_or_else(|error| Err(error.into()), |server| Ok(server.run()));
        Ok(PokedexApp { server, port })
    }
}
