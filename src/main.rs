use pok::PokedexApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = PokedexApp::new(
        "127.0.0.1:8080",
        "https://beta.pokeapi.co/graphql/v1beta".into(),
    )
    .await?;
    app.server?.await
}
