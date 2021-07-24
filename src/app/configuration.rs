use config::{Config, File};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub pokeapi_service: PokeApiServiceSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PokeApiServiceSettings {
    pub url: String,
}

impl ApplicationSettings {
    pub fn binding_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Load the configuration from the directory: `configuration`.
///
/// It fails if the `configuration/local` file is missing or has invalid fields
///
/// # Examples
///
/// ```rust
/// use pok::load_configuration;
///
/// assert!(load_configuration().is_ok());
/// ```
pub fn load_configuration() -> anyhow::Result<Settings> {
    let mut config = Config::new();
    config.merge(File::with_name("configuration/local").required(true))?;
    config.merge(config::Environment::with_prefix("app").separator("__"))?;
    config.try_into().map(Ok)?
}
