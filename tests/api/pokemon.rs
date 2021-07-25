use wiremock::matchers::method;
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::{execute_get_request, spawn_app, PokeApiResponseBuilder};

#[actix_rt::test]
async fn pokemon_returns_200_with_valid_input() {
    let test_app = spawn_app().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(PokeApiResponseBuilder::new().finish()),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;
    let response = execute_get_request(&format!("{}/pokemon/any_pokemon", test_app.address)).await;
    assert_eq!(200, response.status());
}
