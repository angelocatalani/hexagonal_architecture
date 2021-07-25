use actix_web::{web, HttpResponse};

use crate::pokeapi::{PokeapiService, Pokemon};
use crate::routes::errors::PokedexError;
use crate::translated::TranslatedService;
use actix_web::web::Data;

pub async fn pokemon_translated(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
    translated_service: web::Data<TranslatedService>,
) -> Result<HttpResponse, PokedexError> {
    let pokeapi_service_response = pokeapi_service.get_pokemon(name.into_inner()).await?;
    let mut pokemon = pokeapi_service_response.map_err(PokedexError::InvalidRequest)?;

    pokemon.set_description(translate_pokemon_description(translated_service, &pokemon));
    Ok(HttpResponse::Ok().json(pokemon))
}

fn translate_pokemon_description(
    translated_service: Data<TranslatedService>,
    pokemon: &Pokemon,
) -> Option<String> {
    pokemon.description().as_ref().map(|text| {
        if pokemon.has_cave_habitat() || pokemon.is_legendary() {
            translated_service.translate_with_yoda(text)
        } else {
            translated_service.translate_with_shakespeare(text)
        }
    })
}
