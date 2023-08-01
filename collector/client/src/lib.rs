use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GatherRequest {
    pub purls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GatherResponse {
    pub purls: Vec<String>,
    pub vurls: Vec<String>,
}

pub struct Client {
    url: String,
}

impl Client {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn gather(&self, request: GatherRequest) -> Result<GatherResponse, anyhow::Error> {
        let response = reqwest::Client::new()
            .post(self.url.clone())
            .json(&request)
            .send()
            .await?;

        let response: GatherResponse = response.json().await?;

        Ok(response)
    }
}