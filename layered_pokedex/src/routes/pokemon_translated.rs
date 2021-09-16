use actix_web::{web, HttpResponse};
use anyhow::Context;

use crate::cache::CacheService;
use crate::pokeapi::{PokeapiService, Pokemon};
use crate::routes::errors::PokedexError;
use crate::translated::TranslatedService;

#[tracing::instrument(
name = "Getting pokemon with translated description",
skip(name, pokeapi_service, translated_service,cache_service),
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

    // todo: refactor
    if let Ok(description) = cache_service.get(&pokemon_name).await {
        pokemon.set_description(description);
        Ok(HttpResponse::Ok().json(&pokemon))
    } else {
        let translated_description = translate_pokemon_description(translated_service, &pokemon)
            .await
            .context("Failed to translate pokemon description");
        match translated_description {
            Ok(description) => {
                cache_service
                    .set(&pokemon_name, description.as_deref())
                    .await
                    .map_err(|_| tracing::warn!("Failed to update redis cache"))
                    .ok();
                pokemon.set_description(description);
                Ok(HttpResponse::Ok().json(&pokemon))
            }
            Err(_) => Ok(HttpResponse::Ok().json(&pokemon)),
        }
    }
}

#[tracing::instrument(
name = "Retrieving pokemon",
skip(pokeapi_service),
fields(
name = % name,
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
skip(translated_service),
fields(
pokemon = % format ! ("{:#?}", pokemon),
)
)]
async fn translate_pokemon_description(
    translated_service: web::Data<TranslatedService>,
    pokemon: &Pokemon,
) -> anyhow::Result<Option<String>> {
    match pokemon.description() {
        None => Ok(None),
        Some(text) => {
            let new_description = if pokemon.has_cave_habitat_or_is_legendary() {
                translated_service.translate_with_yoda(text).await?
            } else {
                translated_service.translate_with_shakespeare(text).await?
            };
            Ok(Some(new_description))
        }
    }
}
