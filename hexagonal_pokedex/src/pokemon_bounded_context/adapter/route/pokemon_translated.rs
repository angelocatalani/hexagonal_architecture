use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::pokemon_bounded_context::adapter::out::{FuntranslationApi, PokeApi, RedisCache};
use crate::pokemon_bounded_context::adapter::route::error::PokedexError;
use crate::pokemon_bounded_context::port::service::{PokemonInfo, PokemonTranslator};

pub async fn pokemon_translated(
    name: web::Path<String>,
    pokemon_info: web::Data<PokemonInfo<PokeApi>>,
    pokemon_translator: web::Data<
        PokemonTranslator<FuntranslationApi, FuntranslationApi, RedisCache, RedisCache>,
    >,
) -> Result<HttpResponse, PokedexError> {
    let pokemon = pokemon_info
        .into_inner()
        .get(name.into_inner())
        .await
        .context("Failed to retrieve pokemon")
        .map_err(PokedexError::InvalidRequest)?;
    let translated_pokemon = pokemon_translator
        .into_inner()
        .translate(pokemon)
        .await
        .context("Failed to translate pokemon description")?;
    Ok(HttpResponse::Ok().json(&translated_pokemon))
}
