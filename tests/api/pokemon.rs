use reqwest::StatusCode;
use wiremock::matchers::{header, method};
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::spawn_app;

const POKEMON_NAME: &str = "mewtwo";

#[actix_rt::test]
async fn pokemon_executes_request_to_pokeapi() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    let pokemon_endpoint = format!("{}/pokemon/{}", test_app.address, POKEMON_NAME);
    let client = reqwest::Client::new();
    let response = client.post(&pokemon_endpoint).send().await.unwrap();
    assert_eq!(StatusCode::OK, response.status().as_u16());
}
