#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait YodaTranslator {
    async fn to_yoda(&self, text: &str) -> anyhow::Result<String>;
}
