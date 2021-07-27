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
    pub translated_server: MockServer,
}

pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    let pokeapi_server = MockServer::start().await;
    let translated_server = MockServer::start().await;

    let mut config = load_configuration().unwrap();
    config.application.port = 0;
    config.pokeapi_service.url = pokeapi_server.uri().parse().unwrap();
    config.translated_service.url = translated_server.uri().parse().unwrap();

    let app = PokedexApp::new(config).await.unwrap();

    tokio::spawn(app.server.unwrap());

    TestApp {
        address: format!("http://127.0.0.1:{}", app.port),
        pokeapi_server,
        translated_server,
    }
}

pub struct PokeApiResponseBuilder<'a> {
    habitat_name: &'a str,
    is_legendary: bool,
    without_pokemon: bool,
}

impl<'a> PokeApiResponseBuilder<'a> {
    pub fn new() -> PokeApiResponseBuilder<'a> {
        Self {
            habitat_name: "any_habitat",
            is_legendary: true,
            without_pokemon: false,
        }
    }
    pub fn with_habitat(&mut self, name: &'a str) -> &mut Self {
        self.habitat_name = name;
        self
    }
    pub fn with_legendary_status(&mut self, is_legendary: bool) -> &mut Self {
        self.is_legendary = is_legendary;
        self
    }
    pub fn without_pokemon(&mut self) -> &mut Self {
        self.without_pokemon = true;
        self
    }
    pub fn finish(&self) -> Value {
        if self.without_pokemon {
            json!(
                {
                   "data":{
                      "info":[]
                   }
                }
            )
        } else {
            json!(
                {
                   "data":{
                      "info":[
                         {
                            "name":"any_pokemon",
                            "habitat":{
                               "name":self.habitat_name
                            },
                            "descriptions":[
                               {
                                  "flavor_text":"any_description"
                               }
                            ],
                            "is_legendary":self.is_legendary
                         }
                      ]
                   }
                }
            )
        }
    }
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
