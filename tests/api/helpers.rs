use serde_json::Value;
use wiremock::MockServer;

use pok::{load_configuration, setup_tracing, PokedexApp};

lazy_static::lazy_static! {
 static ref TRACING: () = setup_tracing("test".into(),"debug".into());
}

pub struct TestApp {
    pub pokeapi_server: MockServer,
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);
    let pokeapi_server = MockServer::start().await;
    let mut config = load_configuration().unwrap();
    config.application.port = 0;
    config.pokeapi_service.url = pokeapi_server.uri().parse().unwrap();
    let app = PokedexApp::new(config).await.unwrap();
    tokio::spawn(app.server.unwrap());
    TestApp {
        pokeapi_server,
        address: format!("http://127.0.0.1:{}", app.port),
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
