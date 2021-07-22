pub struct PokemonInfo;
pub mod pokemon_info {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PokemonInfo";
    pub const QUERY : & str = "query PokemonInfo($name:String!) {\n    pokemon_v2_pokemonspecies(where: {name: {_eq: $name}}) {\n        name\n        pokemon_v2_pokemonhabitat {\n            name\n        }\n        pokemon_v2_pokemonspeciesflavortexts(limit: 1, where: {pokemon_v2_language: {iso639: {_eq: \"en\"}}}) {\n            flavor_text\n        }\n        is_legendary\n    }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        pub name: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub pokemon_v2_pokemonspecies: Vec<PokemonInfoPokemonV2Pokemonspecies>,
    }
    #[derive(Deserialize)]
    pub struct PokemonInfoPokemonV2Pokemonspecies {
        pub name: String,
        pub pokemon_v2_pokemonhabitat:
            Option<PokemonInfoPokemonV2PokemonspeciesPokemonV2Pokemonhabitat>,
        pub pokemon_v2_pokemonspeciesflavortexts:
            Vec<PokemonInfoPokemonV2PokemonspeciesPokemonV2Pokemonspeciesflavortexts>,
        pub is_legendary: Boolean,
    }
    #[derive(Deserialize)]
    pub struct PokemonInfoPokemonV2PokemonspeciesPokemonV2Pokemonhabitat {
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct PokemonInfoPokemonV2PokemonspeciesPokemonV2Pokemonspeciesflavortexts {
        pub flavor_text: String,
    }
}
impl graphql_client::GraphQLQuery for PokemonInfo {
    type Variables = pokemon_info::Variables;
    type ResponseData = pokemon_info::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: pokemon_info::QUERY,
            operation_name: pokemon_info::OPERATION_NAME,
        }
    }
}
