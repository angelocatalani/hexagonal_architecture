use std::convert::TryInto;
use std::time::Duration;

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
    pub fn new(url: Url, timeout_second: u64) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout_second))
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
    use serde_json::{json, Value};
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn pokeapi_service_successfully_parses_response_with_all_fields() {
        let server = MockServer::start().await;

        let pokemon_name = "mewtwo";
        let habitat = "rare";
        let description = "It was created by a scientist";
        let is_legendary = true;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(build_pokeapi_response(
                    pokemon_name,
                    Some(habitat),
                    &[description],
                    is_legendary,
                )),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), 10).unwrap();

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

    #[tokio::test]
    async fn pokeapi_service_successfully_parses_response_with_missing_fields() {
        let server = MockServer::start().await;

        let pokemon_name = "mewtwo";
        let is_legendary = true;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(build_pokeapi_response(
                    pokemon_name,
                    None,
                    &[],
                    is_legendary,
                )),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), 10).unwrap();

        let correct_response = json!({
            "name": pokemon_name,
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

    #[tokio::test]
    async fn pokeapi_service_handles_correctly_empty_response_without_pokemons() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "data":{
                        "info":[]
                    }
                }
            )))
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), 10).unwrap();

        assert!(service
            .get_pokemon("any_pokemon".into())
            .await
            .unwrap()
            .is_err());
    }

    #[tokio::test]
    async fn pokeapi_service_handles_correctly_error_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "error": [
                        "pokeapi_internal_error"
                    ]
                }
            )))
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), 10).unwrap();

        assert!(service
            .get_pokemon("any_pokemon".into())
            .await
            .unwrap()
            .is_err());
    }

    #[tokio::test]
    async fn pokeapi_service_handles_correctly_unexpected_fields_inside_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "unexpected_field": "unexpected_value"
                }
            )))
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), 10).unwrap();
        assert!(service
            .get_pokemon("any_pokemon".into())
            .await
            .unwrap()
            .is_err());
    }

    #[tokio::test]
    async fn pokeapi_service_handles_correctly_http_timeout() {
        let timeout_seconds = 1;
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200).set_delay(Duration::from_secs(timeout_seconds + 1)),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), timeout_seconds).unwrap();
        assert!(service.get_pokemon("any_pokemon".into()).await.is_err());
    }

    #[tokio::test]
    async fn pokeapi_service_handles_correctly_http_error() {
        let timeout_seconds = 1;
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let service = PokeapiService::new(server.uri().parse().unwrap(), timeout_seconds).unwrap();
        assert!(service.get_pokemon("any_pokemon".into()).await.is_err());
    }

    fn build_pokeapi_response(
        pokemon_name: &str,
        habitat: Option<&str>,
        descriptions: &[&str],
        is_legendary: bool,
    ) -> Value {
        let flavor_texts = descriptions
            .iter()
            .map(|d| json!({ "flavor_text": d }))
            .collect::<Vec<Value>>();

        match habitat {
            Some(habitat_name) => {
                json!(
                    {
                       "data":{
                          "info":[
                             {
                                "name":pokemon_name,
                                "habitat":{
                                   "name":habitat_name
                                },
                                 "descriptions": flavor_texts,
                                "is_legendary":is_legendary
                             }
                          ]
                       }
                    }
                )
            }
            None => {
                json!(
                    {
                       "data":{
                          "info":[
                             {
                                "name":pokemon_name,
                                 "descriptions": flavor_texts,
                                "is_legendary":is_legendary
                             }
                          ]
                       }
                    }
                )
            }
        }
    }
}
