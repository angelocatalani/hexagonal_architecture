use anyhow::{Context, Error};
use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, Url};
use std::convert::TryInto;

use crate::pokeapi::pokemon::{GqlPokemon, GqlPokemonResponse, GqlPokemonVariables};
use crate::pokeapi::Pokemon;

pub struct PokeapiService {
    client: Client,
    url: Url,
}

impl PokeapiService {
    pub fn new(url: Url) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .build()
                .context(format!("Error creating pokeapi client with:\nurl: {}", url,))?,
            url,
        })
    }

    pub async fn get_pokemon(&self, name: String) -> anyhow::Result<Result<Pokemon, String>> {
        let graphql_response = self.execute_gql_pokemon_query(name).await?;

        PokeapiService::parse_gql_pokemon_query(graphql_response)
    }

    fn parse_gql_pokemon_query(
        graphql_response: Response<GqlPokemonResponse>,
    ) -> anyhow::Result<Result<Pokemon, String>> {
        let gqp_errors = graphql_response.errors;
        let graphql_response_parsed = graphql_response
            .data
            .ok_or_else(|| format!("Empty response with errors: {:?}", gqp_errors));

        Ok(graphql_response_parsed.and_then(TryInto::try_into))
    }

    async fn execute_gql_pokemon_query(
        &self,
        name: String,
    ) -> anyhow::Result<Response<GqlPokemonResponse>> {
        let request_body = GqlPokemon::build_query(GqlPokemonVariables { name });

        let response = self
            .client
            .post(self.url.as_str())
            .json(&request_body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to send request")?;

        let graphql_response: Response<GqlPokemonResponse> = response
            .json()
            .await
            .context("Failed to serialize graphql response")?;

        Ok(graphql_response)
    }
}
