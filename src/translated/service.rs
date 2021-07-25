use std::time::Duration;

use anyhow::Context;
use reqwest::{Client, Url};

pub struct TranslatedService {
    client: Client,
    url: Url,
}

impl TranslatedService {
    pub fn new(url: Url, timeout_second: u64) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout_second))
                .build()
                .context(format!("Error creating client with:\nurl: {}", url,))?,
            url,
        })
    }
    pub async fn translate_with_yoda(&self, text: &str) -> anyhow::Result<String> {
        Ok(text.to_string())
    }
    pub async fn translate_with_shakespeare(&self, text: &str) -> anyhow::Result<String> {
        Ok(text.to_string())
    }
}
