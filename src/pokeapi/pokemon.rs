use std::convert::TryFrom;

use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql_api/schema.graphql",
    query_path = "graphql_api/gql_pokemon.graphql",
    response_derives = "Clone"
)]
pub struct GqlPokemon;

pub type GqlPokemonVariables = gql_pokemon::Variables;

pub type GqlPokemonResponse = gql_pokemon::ResponseData;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    habitat: Option<String>,
    is_legendary: bool,
    name: String,
}

impl TryFrom<Response<GqlPokemonResponse>> for Pokemon {
    type Error = String;
    fn try_from(graphql_response: Response<GqlPokemonResponse>) -> Result<Self, Self::Error> {
        let gql_errors = graphql_response.errors;
        let response_data = graphql_response
            .data
            .ok_or_else(|| format!("Empty response with errors: {:?}", gql_errors))?;

        let gql_pokemon_info = response_data
            .info
            .first()
            .ok_or_else(|| "Pokemon not found".to_string())?;

        Ok(Pokemon {
            description: gql_pokemon_info
                .descriptions
                .first()
                .map(|d| d.flavor_text.clone()),
            habitat: gql_pokemon_info.habitat.as_ref().map(|h| h.name.clone()),
            is_legendary: gql_pokemon_info.is_legendary,
            name: gql_pokemon_info.name.clone(),
        })
    }
}
