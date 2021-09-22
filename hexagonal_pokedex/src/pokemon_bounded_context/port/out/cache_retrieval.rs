#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait CacheRetrieval {
    async fn get(&self, pokemon_name: &str) -> anyhow::Result<Option<String>>;
}
