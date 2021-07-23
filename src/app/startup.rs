use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;

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
    query_path = "graphql_api/pokemon_info.graphql"
)]
struct PokemonInfo;

async fn pokemon(name: web::Path<String>, pokeapi_url: web::Data<String>) -> HttpResponse {
    let request_body = PokemonInfo::build_query(pokemon_info::Variables {
        name: name.into_inner(),
    });
    let client = reqwest::Client::new();
    let graphql_response = client
        .post(pokeapi_url.as_str())
        .json(&request_body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    let response_data: Response<pokemon_info::ResponseData> =
        graphql_response.json().await.unwrap();
    let d = response_data.data.unwrap();
    let pokemon_info = d.pokemons.first().unwrap();

    let pokemon_name = &pokemon_info.name;
    let description = &pokemon_info.descriptions.first().unwrap().flavor_text;
    let habitat = &pokemon_info.habitat.as_ref().unwrap().name;
    let is_legendary = &pokemon_info.is_legendary;
    HttpResponse::Ok().json(json!({
        "name":pokemon_name,
        "description":description,
        "habitat": habitat,
        "isLegendary": is_legendary

    }))
}
