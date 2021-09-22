use crate::pokemon_bounded_context::domain::Pokemon;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait PokemonRetrieval {
    async fn get(&self, pokemon_name: String) -> anyhow::Result<Pokemon>;
}
