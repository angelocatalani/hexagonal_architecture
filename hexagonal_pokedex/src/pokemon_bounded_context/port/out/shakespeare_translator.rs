#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ShakespeareTranslator {
    async fn to_shakespeare(&self, text: &str) -> anyhow::Result<String>;
}
