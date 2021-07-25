use std::time::Duration;

use anyhow::Context;
use reqwest::{Client, Response, Url};
use serde_json::{json, Value};

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
        self.execute_post_request(self.url.join("yoda.json")?.as_str(), json!({}))
            .await?;
        Ok(text.to_string())
    }
    pub async fn translate_with_shakespeare(&self, text: &str) -> anyhow::Result<String> {
        self.execute_post_request(self.url.join("shakespeare.json")?.as_str(), json!({}))
            .await?;
        Ok(text.to_string())
    }

    async fn execute_post_request(
        &self,
        endpoint: &str,
        json_body: Value,
    ) -> anyhow::Result<Response> {
        self.client
            .post(endpoint)
            .json(&json_body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to send request")
    }
}
