use actix_web::{web, HttpResponse};

use crate::pokeapi::PokeapiService;
use crate::routes::errors::PokedexError;

pub async fn pokemon_translated(
    name: web::Path<String>,
    pokeapi_service: web::Data<PokeapiService>,
) -> Result<HttpResponse, PokedexError> {
    unimplemented!()
}
