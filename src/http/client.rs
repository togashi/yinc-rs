/// HTTP client for fetching remote resources
use crate::Result;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let content = response.text().await?;
        Ok(content)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
