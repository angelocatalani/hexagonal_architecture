use anyhow::Context;

use pok::{load_configuration, PokedexApp};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let app = PokedexApp::new(load_configuration().context("Failed to load local configuration")?)
        .await
        .context("Failed to instantiate PokeapiApp")?;
    app.server
        .context("Failed to start server")?
        .await
        .map_err(Into::into)
}
