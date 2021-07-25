use reqwest::StatusCode;
use serde_json::json;
use wiremock::matchers::method;
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::{spawn_app, valid_pokeapi_response};

#[actix_rt::test]
async fn pokemon_returns_200_with_valid_input() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_pokeapi_response()))
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    let pokemon_endpoint_with_valid_input = format!("{}/pokemon/any_pokemon", test_app.address);
    let client = reqwest::Client::new();
    let response = client
        .get(&pokemon_endpoint_with_valid_input)
        .send()
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, response.status());
}
