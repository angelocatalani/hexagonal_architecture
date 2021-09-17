use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::cache::CacheService;
use crate::pokeapi::{PokeapiService, Pokemon};
use crate::routes::errors::PokedexError;
use crate::translated::TranslatedService;

#[tracing::instrument(
name = "Getting pokemon with translated description",
skip(name, pokeapi_service, translated_service, cache_service),
fields(
pokemon_name = % name.as_str(),
)
)]
pub async fn pokemon_translated(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
    translated_service: web::Data<TranslatedService>,
    cache_service: web::Data<CacheService>,
) -> Result<HttpResponse, PokedexError> {
    let pokemon_name = name.into_inner();
    let mut pokemon = retrieve_pokemon(pokemon_name.clone(), pokeapi_service).await?;

    let translated_description =
        translate_pokemon_description(translated_service, cache_service, &pokemon)
            .await
            .context("Failed to translate pokemon description");

    match translated_description {
        Ok(description) => {
            pokemon.set_description(description);
            Ok(HttpResponse::Ok().json(&pokemon))
        }
        Err(_) => Ok(HttpResponse::Ok().json(&pokemon)),
    }
}

#[tracing::instrument(
name = "Retrieving pokemon",
skip(pokeapi_service),
fields(
pokemon_name = % name,
)
)]
async fn retrieve_pokemon(
    name: String,
    pokeapi_service: web::Data<PokeapiService>,
) -> Result<Pokemon, PokedexError> {
    let pokeapi_service_response = pokeapi_service.get_pokemon(name).await?;
    pokeapi_service_response.map_err(PokedexError::InvalidRequest)
}

#[tracing::instrument(
name = "Translating pokemon description",
skip(translated_service, cache_service),
fields(
pokemon = % format ! ("{:#?}", pokemon),
)
)]
async fn translate_pokemon_description(
    translated_service: web::Data<TranslatedService>,
    cache_service: web::Data<CacheService>,
    pokemon: &Pokemon,
) -> anyhow::Result<Option<String>> {
    match pokemon.description() {
        None => Ok(None),
        Some(text) => match cache_service.get(pokemon.name()).await {
            Ok(translated_description) => Ok(Some(translated_description)),
            Err(_) => {
                let translated_description = if pokemon.has_cave_habitat_or_is_legendary() {
                    translated_service.translate_with_yoda(text).await?
                } else {
                    translated_service.translate_with_shakespeare(text).await?
                };
                cache_service
                    .set(pokemon.name(), &translated_description)
                    .await
                    .map_err(|e| {
                        tracing::warn!("Failed to update the descriptions cache.\n{:?}", e)
                    })
                    .ok();
                Ok(Some(translated_description))
            }
        },
    }
}
