use pok::PokedexApp;

pub async fn spawn_app() -> u16 {
    let app = PokedexApp::new("127.0.0.1:8080").await.unwrap();
    tokio::spawn(app.server.unwrap());
    app.port
}
