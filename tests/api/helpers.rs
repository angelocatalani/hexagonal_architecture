use reqwest::Response;
use serde_json::{json, Value};
use wiremock::MockServer;

use pok::{load_configuration, setup_tracing, PokedexApp};

lazy_static::lazy_static! {
 static ref TRACING: () = setup_tracing("test".into(),"debug".into());
}

pub struct TestApp {
    pub address: String,
    pub pokeapi_server: MockServer,
    pub translation_server: MockServer,
}

pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    let pokeapi_server = MockServer::start().await;
    let translation_server = MockServer::start().await;

    let mut config = load_configuration().unwrap();
    config.application.port = 0;
    config.pokeapi_service.url = pokeapi_server.uri().parse().unwrap();
    config.translation_service.url = translation_server.uri().parse().unwrap();

    let app = PokedexApp::new(config).await.unwrap();

    tokio::spawn(app.server.unwrap());

    TestApp {
        address: format!("http://127.0.0.1:{}", app.port),
        pokeapi_server,
        translation_server,
    }
}

pub fn valid_pokeapi_response() -> Value {
    json!(
        {
           "data":{
              "info":[
                 {
                    "name":"any_pokemon",
                    "habitat":{
                       "name":"any_habitat"
                    },
                    "descriptions":[
                       {
                          "flavor_text":"any_description"
                       }
                    ],
                    "is_legendary":true
                 }
              ]
           }
        }
    )
}

pub fn valid_translation_response() -> Value {
    json!(
        {
            "success":{
                "total": 1
            },
            "contents": {
                "translated": "any_text_translated",
                "text": "any_text",
                "translation": "any_translation"
            }
        }
    )
}

pub async fn execute_get_request(endpoint: &str) -> Response {
    let client = reqwest::Client::new();
    client.get(endpoint).send().await.unwrap()
}
