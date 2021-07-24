use actix_web::{web, HttpResponse};
use anyhow::Context;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;

use crate::app::errors::PokedexError;

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
                .map(|d| d.flavor_text.clone()),
            habitat: gql_pokemon_info.habitat.map(|h| h.name),
            is_legendary: gql_pokemon_info.is_legendary,
            name: gql_pokemon_info.name,
        }
    }
}

pub async fn pokemon(
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
