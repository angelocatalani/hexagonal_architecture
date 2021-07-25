use reqwest::StatusCode;
use wiremock::matchers::method;
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::{
    execute_get_request, spawn_app, valid_pokeapi_response, valid_translation_response,
};

#[actix_rt::test]
async fn pokemon_translated_returns_200_with_valid_input() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_pokeapi_response()))
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translation_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(StatusCode::OK, response.status());
}