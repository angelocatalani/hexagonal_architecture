use pok::PokedexApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = PokedexApp::new("127.0.0.1:8080").await?;
    app.server?.await
}
