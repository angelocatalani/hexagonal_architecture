use anyhow::Context;

use pok::{load_configuration, setup_tracing, PokedexApp};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing("pokedex".into(), "info".into());

    let config = load_configuration().context("Failed to load local configuration")?;
    tracing::info!("spawning server on url: {}", config.binding_address());

    let app = PokedexApp::new(config)
        .await
        .context("Failed to instantiate PokeapiApp")?;

    app.server
        .context("Failed to start server")?
        .await
        .map_err(Into::into)
}
