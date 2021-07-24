use anyhow::Context;
use reqwest::{Client, Url};
use serde::Serialize;

use graphql_client::{GraphQLQuery, Response};
use std::convert::{TryFrom, TryInto};

#[derive(Clone)]
pub struct PokeapiService {
    client: Client,
    url: Url,
}

impl PokeapiService {
    pub fn new(url: Url) -> Result<Self, anyhow::Error> {
        Ok(Self {
            client: Client::builder()
                .build()
                .context(format!("Error creating pokeapi client with:\nurl: {}", url,))?,
            url,
        })
    }

    pub async fn get_pokemon(&self, name: String) -> anyhow::Result<Result<Pokemon, String>> {
        let request_body = GqlPokemon::build_query(gql_pokemon::Variables { name });

        let response = self
            .client
            .post(self.url.as_str())
            .json(&request_body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to send request")?;

        let graphql_response: Response<gql_pokemon::ResponseData> = response
            .json()
            .await
            .context("Failed to serialize graphql response")?;

        let gqp_errors = graphql_response.errors;
        let graphql_response_parsed = graphql_response
            .data
            .ok_or_else(|| format!("Empty response with errors: {:?}", gqp_errors));

        Ok(graphql_response_parsed.and_then(TryInto::try_into))
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
