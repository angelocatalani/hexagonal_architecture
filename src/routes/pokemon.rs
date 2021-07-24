use actix_web::{web, HttpResponse};

use crate::pokeapi::PokeapiService;
use crate::routes::errors::PokedexError;

pub async fn pokemon(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
) -> Result<HttpResponse, PokedexError> {
    let pokeapi_service_response = pokeapi_service.get_pokemon(name.into_inner()).await?;

    let pokemon = pokeapi_service_response.map_err(|e| PokedexError::InvalidRequest(e))?;

    Ok(HttpResponse::Ok().json(pokemon))
}
