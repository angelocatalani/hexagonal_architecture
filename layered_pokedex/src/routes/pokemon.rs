use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::pokeapi::PokeapiService;
use crate::routes::errors::PokedexError;

#[tracing::instrument(
name = "Getting pokemon info",
skip(name, pokeapi_service),
fields(
pokemon_name = % name.as_str(),
)
)]
pub async fn pokemon(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
) -> Result<HttpResponse, PokedexError> {
    let pokeapi_service_response = pokeapi_service
        .get_pokemon(name.into_inner())
        .await
        .context("Failed to retrieve pokemon")?;

    let pokemon = pokeapi_service_response.map_err(PokedexError::InvalidRequest)?;

    Ok(HttpResponse::Ok().json(pokemon))
}
