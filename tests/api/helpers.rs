use wiremock::MockServer;

use pok::PokedexApp;

pub struct TestApp {
    pub pokeapi_server: MockServer,
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    let pokeapi_server = MockServer::start().await;
    let app = PokedexApp::new("127.0.0.1:0", pokeapi_server.uri())
        .await
        .unwrap();
    tokio::spawn(app.server.unwrap());
    TestApp {
        pokeapi_server,
        address: format!("http://127.0.0.1:{}", app.port),
    }
}
