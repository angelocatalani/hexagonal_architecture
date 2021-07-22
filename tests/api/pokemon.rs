use reqwest::StatusCode;
use serde_json::{json, Value};
use wiremock::matchers::{header, method};
use wiremock::{Mock, ResponseTemplate};

use crate::api::helpers::spawn_app;

const POKEMON_NAME: &str = "mewtwo";

#[actix_rt::test]
async fn pokemon_parses_correctly_pokeapi_response() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(
            {
               "data":{
                  "pokemon_v2_pokemonspecies":[
                     {
                        "name":"mewtwo",
                        "pokemon_v2_pokemonhabitat":{
                           "name":"rare"
                        },
                        "pokemon_v2_pokemonspeciesflavortexts":[
                           {
                              "flavor_text":"It was created by a scientist"
                           }
                        ],
                        "is_legendary":true
                     }
                  ]
               }
            }
        )))
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    let pokemon_endpoint = format!("{}/pokemon/{}", test_app.address, POKEMON_NAME);
    let client = reqwest::Client::new();
    let response = client.get(&pokemon_endpoint).send().await.unwrap();
    let correct = json!({
        "name": "mewtwo",
        "habitat": "rare",
        "description":"It was created by a scientist",
        "isLegendary":true

    });
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(correct, response.json::<Value>().await.unwrap());
}

#[actix_rt::test]
async fn pokemon_returns_404_with_invalid_pokeapi_response() {
    let test_app = spawn_app().await;

    Mock::given(method("POST"))
        .and(header("Content-Type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&test_app.pokeapi_server)
        .await;

    let pokemon_endpoint = format!("{}/pokemon/{}", test_app.address, POKEMON_NAME);
    let client = reqwest::Client::new();
    let response = client.get(&pokemon_endpoint).send().await.unwrap();
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
