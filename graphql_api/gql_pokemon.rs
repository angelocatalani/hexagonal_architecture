pub struct GqlPokemon;
pub mod gql_pokemon {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GqlPokemon";
    pub const QUERY : & str = "query GqlPokemon($name:String!) {\n    info: pokemon_v2_pokemonspecies(where: {name: {_eq: $name}}) {\n        name\n        habitat: pokemon_v2_pokemonhabitat {\n            name\n        }\n        descriptions: pokemon_v2_pokemonspeciesflavortexts(limit: 1, where: {pokemon_v2_language: {iso639: {_eq: \"en\"}}}) {\n            flavor_text\n        }\n        is_legendary\n    }\n}\n" ;
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
        pub info: Vec<GqlPokemonInfo>,
    }
    #[derive(Deserialize)]
    pub struct GqlPokemonInfo {
        pub name: String,
        pub habitat: Option<GqlPokemonInfoHabitat>,
        pub descriptions: Vec<GqlPokemonInfoDescriptions>,
        pub is_legendary: Boolean,
    }
    #[derive(Deserialize)]
    pub struct GqlPokemonInfoHabitat {
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct GqlPokemonInfoDescriptions {
        pub flavor_text: String,
    }
}
impl graphql_client::GraphQLQuery for GqlPokemon {
    type Variables = gql_pokemon::Variables;
    type ResponseData = gql_pokemon::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: gql_pokemon::QUERY,
            operation_name: gql_pokemon::OPERATION_NAME,
        }
    }
}
