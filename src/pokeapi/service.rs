use std::convert::TryInto;

use anyhow::Context;
use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, Url};

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

        Ok(graphql_response.try_into())
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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use reqwest::{StatusCode, Url};
    use serde::Serialize;
    use serde_json::{json, Value};
    use wiremock::matchers::body_json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn pokeapi_service_successfully_parses_valid_response() {
        let server = MockServer::start().await;

        let pokemon_name = "mewtwo";
        let habitat = "rare";
        let description = "It was created by a scientist";
        let is_legendary = true;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(build_pokeapi_response(
                    pokemon_name,
                    habitat,
                    description,
                    is_legendary,
                )),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap()).unwrap();

        let correct_response = json!({
            "name": pokemon_name,
            "habitat": habitat,
            "description":description,
            "isLegendary":is_legendary

        });

        assert_eq!(
            correct_response,
            json!(service
                .get_pokemon(pokemon_name.to_string())
                .await
                .unwrap()
                .unwrap())
        );
    }

    fn build_pokeapi_response(
        pokemon_name: &str,
        habitat: &str,
        flavor_text: &str,
        is_legendary: bool,
    ) -> Value {
        json!(
            {
               "data":{
                  "info":[
                     {
                        "name":pokemon_name,
                        "habitat":{
                           "name":habitat
                        },
                        "descriptions":[
                           {
                              "flavor_text":flavor_text
                           }
                        ],
                        "is_legendary":is_legendary
                     }
                  ]
               }
            }
        )
    }
}
