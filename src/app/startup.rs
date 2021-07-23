use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Context;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;

use crate::app::errors::PokedexError;

pub struct PokedexApp {
    pub server: Result<Server, std::io::Error>,
    pub port: u16,
}

impl PokedexApp {
    pub async fn new(
        binding_address: &str,
        pokeapi_url: String,
    ) -> Result<PokedexApp, std::io::Error> {
        let tcp_listener = TcpListener::bind(binding_address)?;
        let port = tcp_listener.local_addr().unwrap().port();
        let pokeapi_url_data = web::Data::new(pokeapi_url);
        let server = HttpServer::new(move || {
            App::new()
                .route("/health_check", web::get().to(HttpResponse::Ok))
                .route("/pokemon/{name}", web::get().to(pokemon))
                .app_data(pokeapi_url_data.clone())
        })
        .listen(tcp_listener)
        .map(HttpServer::run);
        Ok(PokedexApp { server, port })
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql_api/schema.graphql",
    query_path = "graphql_api/gql_pokemon.graphql",
    response_derives = "Clone"
)]
struct GqlPokemon;

#[derive(Serialize)]
struct Pokemon {
    description: Option<String>,
    habitat: Option<String>,
    #[serde(rename = "isLegendary")]
    is_legendary: bool,
    name: String,
}

impl<'a> From<gql_pokemon::GqlPokemonInfo> for Pokemon {
    fn from(gql_pokemon_info: gql_pokemon::GqlPokemonInfo) -> Self {
        Pokemon {
            description: gql_pokemon_info
                .descriptions
                .first()
                .and_then(|d| Some(d.flavor_text.clone())),
            habitat: gql_pokemon_info.habitat.and_then(|h| Some(h.name)),
            is_legendary: gql_pokemon_info.is_legendary,
            name: gql_pokemon_info.name,
        }
    }
}

async fn pokemon(
    name: web::Path<String>,
    pokeapi_url: web::Data<String>,
) -> Result<HttpResponse, PokedexError> {
    let request_body = GqlPokemon::build_query(gql_pokemon::Variables {
        name: name.into_inner(),
    });
    let client = reqwest::Client::new();
    let response = client
        .post(pokeapi_url.as_str())
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("Failed to send request to pokeapi server")?;

    let graphql_response: Response<gql_pokemon::ResponseData> = response
        .json()
        .await
        .context("Failed to serialize graphql response")?;

    let graphql_data = graphql_response.data.as_ref().ok_or_else(|| {
        PokedexError::InvalidRequest(format!(
            "Empty response with errors: {:?}",
            graphql_response.errors
        ))
    })?;
    let first_pokemon = graphql_data
        .info
        .first()
        .ok_or_else(|| PokedexError::InvalidRequest("Pokemon not found".to_string()))?;

    Ok(HttpResponse::Ok().json(Pokemon::from(first_pokemon.clone())))
}
