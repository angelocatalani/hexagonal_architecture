use anyhow::Context;
use redis::AsyncCommands;

pub struct CacheService {
    connection_manager: redis::aio::ConnectionManager,
}

impl CacheService {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(url).context("Error creating Redis client")?;
        let connection_manager = client
            .get_tokio_connection_manager()
            .await
            .context("Error creating the connection manager")?;

        Ok(Self { connection_manager })
    }
    pub async fn get(&self, pokemon_name: &str) -> anyhow::Result<String> {
        let mut connection = self.connection_manager.clone();
        connection
            .get(pokemon_name)
            .await
            .with_context(|| format!("Error retrieving pokemon: {}", pokemon_name))
    }
    pub async fn set(
        &self,
        pokemon_name: &str,
        pokemon_description: &str,
    ) -> anyhow::Result<String> {
        let mut connection = self.connection_manager.clone();
        connection
            .set(&pokemon_name, pokemon_description)
            .await
            .with_context(|| format!("Error setting description for pokemon: {}", pokemon_name))
    }
}
