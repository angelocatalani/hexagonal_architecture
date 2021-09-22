use std::convert::TryFrom;

use graphql_client::Response;

use crate::pokemon_bounded_context::domain::Pokemon;

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "src/pokemon_bounded_context/adapter/out/poke_api/graphql_api/schema.graphql",
    query_path = "src/pokemon_bounded_context/adapter/out/poke_api/graphql_api/gql_pokemon.graphql"
)]
pub struct GqlPokemon;

pub type GqlPokemonVariables = gql_pokemon::Variables;

pub type GqlPokemonResponse = gql_pokemon::ResponseData;

impl TryFrom<Response<GqlPokemonResponse>> for Pokemon {
    type Error = anyhow::Error;
    fn try_from(graphql_response: Response<GqlPokemonResponse>) -> Result<Self, Self::Error> {
        let gql_errors = graphql_response.errors;

        let response_data = graphql_response
            .data
            .ok_or_else(|| anyhow::anyhow!("Empty response with errors: {:?}", gql_errors))?;

        let gql_pokemon_info = response_data
            .info
            .first()
            .ok_or_else(|| anyhow::anyhow!("Pokemon not found".to_string()))?;
        Ok(Pokemon::new(
            gql_pokemon_info
                .descriptions
                .first()
                .map(|d| d.flavor_text.clone()),
            gql_pokemon_info.habitat.as_ref().map(|h| h.name.clone()),
            gql_pokemon_info.is_legendary,
            gql_pokemon_info.name.clone(),
        ))
    }
}
