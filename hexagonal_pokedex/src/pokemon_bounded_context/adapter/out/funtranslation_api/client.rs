use std::time::Duration;

use anyhow::Context;
use reqwest::{Client, Response, Url};

use crate::pokemon_bounded_context::adapter::out::funtranslation_api::io::{Input, Output};
use crate::pokemon_bounded_context::port::out::{ShakespeareTranslator, YodaTranslator};

pub struct FuntranslationApi {
    client: Client,
    url: Url,
}

#[async_trait::async_trait]
impl YodaTranslator for FuntranslationApi {
    async fn to_yoda(&self, text: &str) -> anyhow::Result<String> {
        let response = self
            .execute_post_request(self.url.join("yoda.json")?.as_str(), text)
            .await?;
        parse_response(response).await
    }
}

#[async_trait::async_trait]
impl ShakespeareTranslator for FuntranslationApi {
    async fn to_shakespeare(&self, text: &str) -> anyhow::Result<String> {
        let response = self
            .execute_post_request(self.url.join("shakespeare.json")?.as_str(), text)
            .await?;
        parse_response(response).await
    }
}

impl FuntranslationApi {
    pub fn new(url: Url, timeout_second: u64) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout_second))
                .build()
                .context(format!("Error creating client with:\nurl: {}", url,))?,
            url,
        })
    }
    async fn execute_post_request(&self, endpoint: &str, text: &str) -> anyhow::Result<Response> {
        self.client
            .post(endpoint)
            .form(&Input { text })
            .send()
            .await
            .context("Failed to send request")
    }
}

async fn parse_response(response: Response) -> anyhow::Result<String> {
    let description = response
        .json::<Output>()
        .await
        .context("Failed to parse the response")?;
    Ok(description.translated())
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn funtranslation_successfully_parses_valid_shakespeare_response() {
        let server = MockServer::start().await;
        let translated_description = "translated_description";

        Mock::given(method("POST"))
            .and(path("/shakespeare.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(translated_valid_response(translated_description)),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = FuntranslationApi::new(server.uri().parse().unwrap(), 10).unwrap();
        assert_eq!(
            translated_description,
            service.to_shakespeare("any_text").await.unwrap()
        );
    }

    #[tokio::test]
    async fn funtranslation_successfully_parses_valid_yoda_response() {
        let server = MockServer::start().await;
        let translated_description = "translated_description";

        Mock::given(method("POST"))
            .and(path("/yoda.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(translated_valid_response(translated_description)),
            )
            .expect(1)
            .mount(&server)
            .await;

        let service = FuntranslationApi::new(server.uri().parse().unwrap(), 10).unwrap();
        assert_eq!(
            translated_description,
            service.to_yoda("any_text").await.unwrap()
        );
    }

    #[tokio::test]
    async fn funtranslation_successfully_handles_invalid_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
            .expect(2)
            .mount(&server)
            .await;

        let service = FuntranslationApi::new(server.uri().parse().unwrap(), 10).unwrap();
        assert!(service.to_yoda("any_text").await.is_err());
        assert!(service.to_shakespeare("any_text").await.is_err());
    }

    #[tokio::test]
    async fn funtranslation_successfully_handles_http_error_response() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .expect(2)
            .mount(&server)
            .await;

        let service = FuntranslationApi::new(server.uri().parse().unwrap(), 10).unwrap();
        assert!(service.to_yoda("any_text").await.is_err());
        assert!(service.to_shakespeare("any_text").await.is_err());
    }

    fn translated_valid_response(translated_description: &str) -> Value {
        json!(
            {
                "success": {
                    "total": 1
                },
                "contents": {
                    "translated": translated_description,
                    "text": "any_text",
                    "translation": "any_translation"
                }
            }
        )
    }
}
