use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::pokemon_bounded_context::adapter::out::PokeApi;
use crate::pokemon_bounded_context::adapter::route::error::PokedexError;
use crate::pokemon_bounded_context::port::service::PokemonInfo;

pub async fn pokemon(
    name: web::Path<String>,
    pokemon_info: web::Data<PokemonInfo<PokeApi>>,
) -> Result<HttpResponse, PokedexError> {
    let pokemon = pokemon_info
        .into_inner()
        .get(name.into_inner())
        .await
        .context("Failed to retrieve pokemon")
        .map_err(PokedexError::InvalidRequest)?;

    Ok(HttpResponse::Ok().json(pokemon))
}
