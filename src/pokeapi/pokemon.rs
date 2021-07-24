use graphql_client::GraphQLQuery;
use serde::Serialize;

use std::convert::TryFrom;

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
    description: Option<String>,
    habitat: Option<String>,
    is_legendary: bool,
    name: String,
}

impl TryFrom<gql_pokemon::ResponseData> for Pokemon {
    type Error = String;
    fn try_from(response_data: gql_pokemon::ResponseData) -> Result<Self, Self::Error> {
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
