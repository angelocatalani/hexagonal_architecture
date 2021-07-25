use std::time::Duration;

use anyhow::Context;
use reqwest::{Client, Response, Url};

use crate::translated::io::{TranslatedInput, TranslatedOutput};

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
        let response = self
            .execute_post_request(self.url.join("yoda.json")?.as_str(), text)
            .await?;
        parse_response(response).await
    }

    pub async fn translate_with_shakespeare(&self, text: &str) -> anyhow::Result<String> {
        let response = self
            .execute_post_request(self.url.join("shakespeare.json")?.as_str(), text)
            .await?;
        parse_response(response).await
    }

    async fn execute_post_request(&self, endpoint: &str, text: &str) -> anyhow::Result<Response> {
        self.client
            .post(endpoint)
            .form(&TranslatedInput { text })
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to send request")
    }
}

async fn parse_response(response: Response) -> anyhow::Result<String> {
    let description = response
        .json::<TranslatedOutput>()
        .await
        .context("Failed to parse the response")?;
    Ok(description.translated)
}
