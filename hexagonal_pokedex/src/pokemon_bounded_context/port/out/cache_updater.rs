#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait CacheUpdater {
    async fn update(&self, pokemon_name: &str, pokemon_description: String) -> anyhow::Result<()>;
}
