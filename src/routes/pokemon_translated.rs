use actix_web::{web, HttpResponse};

use crate::pokeapi::PokeapiService;
use crate::routes::errors::PokedexError;

pub async fn pokemon_translated(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
) -> Result<HttpResponse, PokedexError> {
    let pokeapi_service_response = pokeapi_service.get_pokemon(name.into_inner()).await?;
    let pokemon = pokeapi_service_response.map_err(PokedexError::InvalidRequest)?;
    if pokemon.is_legendary() || pokemon.habitat_is_cave() {
        unimplemented!()
    } else {
        unimplemented!()
    }
}
