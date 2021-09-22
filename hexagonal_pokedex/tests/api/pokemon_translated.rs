use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::{
    execute_get_request, random_pokemon_name, spawn_app, valid_translation_response,
    PokeApiResponseBuilder,
};

#[actix_rt::test]
async fn pokemon_translated_returns_200_with_valid_input() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(PokeApiResponseBuilder::new().finish()),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translated_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(200, response.status());
}

#[actix_rt::test]
async fn pokemon_translated_returns_404_with_non_existent_pokemon() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(PokeApiResponseBuilder::new().without_pokemon().finish()),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(404, response.status());
}

#[actix_rt::test]
async fn pokemon_translated_calls_yoda_translator_for_pokemon_with_cave_habitat() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                PokeApiResponseBuilder::new()
                    .with_habitat("cave")
                    .with_legendary_status(false)
                    .finish(),
            ),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .and(path("yoda.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translated_server)
        .await;

    Mock::given(method("POST"))
        .and(path("shakespeare.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(0)
        .mount(&test_app.translated_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(200, response.status());
}

#[actix_rt::test]
async fn pokemon_translated_calls_yoda_translator_for_legendary_pokemon() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                PokeApiResponseBuilder::new()
                    .with_habitat("not_cave")
                    .with_legendary_status(true)
                    .finish(),
            ),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .and(path("yoda.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translated_server)
        .await;

    Mock::given(method("POST"))
        .and(path("shakespeare.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(0)
        .mount(&test_app.translated_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(200, response.status());
}

#[actix_rt::test]
async fn pokemon_translated_calls_shakespeare_translator_correctly() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                PokeApiResponseBuilder::new()
                    .with_habitat("not_cave")
                    .with_legendary_status(false)
                    .finish(),
            ),
        )
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .and(path("shakespeare.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translated_server)
        .await;

    Mock::given(method("POST"))
        .and(path("yoda.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(0)
        .mount(&test_app.translated_server)
        .await;

    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    assert_eq!(200, response.status());
}

#[actix_rt::test]
async fn pokemon_translated_cache_translation() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                PokeApiResponseBuilder::new()
                    .with_habitat("not_cave")
                    .with_legendary_status(false)
                    .with_name(random_pokemon_name())
                    .finish(),
            ),
        )
        .expect(2)
        .mount(&test_app.pokeapi_server)
        .await;

    Mock::given(method("POST"))
        .and(path("shakespeare.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(valid_translation_response()))
        .expect(1)
        .mount(&test_app.translated_server)
        .await;

    execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;
    let response = execute_get_request(&format!(
        "{}/pokemon/translated/any_pokemon",
        test_app.address
    ))
    .await;

    assert_eq!(200, response.status());
}
