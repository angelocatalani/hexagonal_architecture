use actix_web::{HttpResponse, web};
use actix_web::web::Data;

use crate::pokeapi::{PokeapiService, Pokemon};
use crate::routes::errors::PokedexError;
use crate::translated::TranslatedService;

pub async fn pokemon_translated(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
    translated_service: web::Data<TranslatedService>,
) -> Result<HttpResponse, PokedexError> {
    let pokeapi_service_response = pokeapi_service.get_pokemon(name.into_inner()).await?;
    let mut pokemon = pokeapi_service_response.map_err(PokedexError::InvalidRequest)?;

    let translated_description = translate_pokemon_description(translated_service, &pokemon).await;

    match translated_description {
        Ok(description) => {
            pokemon.set_description(description);
            Ok(HttpResponse::Ok().json(&pokemon))
        }
        Err(_) => {
            Ok(HttpResponse::Ok().json(&pokemon))
        }
    }
}

async fn translate_pokemon_description(
    translated_service: Data<TranslatedService>,
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
