use anyhow::Context;
use redis::AsyncCommands;

use crate::pokemon_bounded_context::port::out::{CacheRetrieval, CacheUpdater};

pub struct RedisCache {
    connection_manager: redis::aio::ConnectionManager,
}

impl RedisCache {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url).context("Error creating Redis client")?;
        let connection_manager = client
            .get_tokio_connection_manager()
            .await
            .context("Error creating the connection manager")?;

        Ok(Self { connection_manager })
    }
}

#[async_trait::async_trait]
impl CacheRetrieval for RedisCache {
    async fn get(&self, pokemon_name: &str) -> anyhow::Result<Option<String>> {
        let mut connection = self.connection_manager.clone();
        connection
            .get(pokemon_name)
            .await
            .with_context(|| format!("Error retrieving pokemon: {}", pokemon_name))
    }
}

#[async_trait::async_trait]
impl CacheUpdater for RedisCache {
    async fn update(&self, pokemon_name: &str, pokemon_description: String) -> anyhow::Result<()> {
        let mut connection = self.connection_manager.clone();
        connection
            .set(pokemon_name, pokemon_description)
            .await
            .with_context(|| format!("Error setting description for pokemon: {}", pokemon_name))
    }
}
