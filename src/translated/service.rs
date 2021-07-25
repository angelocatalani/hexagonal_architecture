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
    pub fn translate(&self, _text: &str) {
        unimplemented!()
    }
}
