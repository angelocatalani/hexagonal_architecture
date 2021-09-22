use anyhow::Context;

use crate::pokemon_bounded_context::domain::Pokemon;
use crate::pokemon_bounded_context::port::out::PokemonRetrieval;

pub struct PokemonInfo<T>
where
    T: PokemonRetrieval,
{
    pokemon_retrieval: T,
}

impl<T> PokemonInfo<T>
where
    T: PokemonRetrieval,
{
    pub fn new(pokemon_retrieval: T) -> Self {
        Self { pokemon_retrieval }
    }
    pub async fn get(&self, pokemon_name: String) -> anyhow::Result<Pokemon> {
        self.pokemon_retrieval
            .get(pokemon_name)
            .await
            .with_context(|| "Failed to get pokemon info")
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::pokemon_bounded_context::domain::Pokemon;
    use crate::pokemon_bounded_context::port::out::MockPokemonRetrieval;
    use crate::pokemon_bounded_context::port::service::pokemon_info::PokemonInfo;

    #[tokio::test]
    async fn retrieve_pokemon_service_calls_the_correct_port() {
        let mut retrieve_pokemon_port = MockPokemonRetrieval::new();
        retrieve_pokemon_port
            .expect_get()
            .with(eq("any_pokemon_name".to_string()))
            .times(1)
            .returning(|_| {
                Ok(Pokemon::new(
                    None,
                    None,
                    false,
                    "any_pokemon_name".to_string(),
                ))
            });
        let retrieve_pokemon_service = PokemonInfo::new(retrieve_pokemon_port);
        retrieve_pokemon_service
            .get("any_pokemon_name".into())
            .await
            .unwrap();
    }
}
