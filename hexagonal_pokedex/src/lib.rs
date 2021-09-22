pub use configuration::settings::load_configuration;
pub use configuration::startup::PokedexApp;
pub use configuration::telemetry::setup_tracing;

mod configuration;
mod pokemon_bounded_context;
