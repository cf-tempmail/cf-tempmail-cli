use anyhow::{anyhow, Result};
use reqwest::Client as HttpClient;
use serde::Deserialize;

pub struct Client {
    base: String,
    http: HttpClient,
}

#[derive(Deserialize)]
pub struct AliasResponse {
    pub email: String,
    pub alias: String,
    pub signature: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: u64,
}

#[derive(Deserialize)]
pub struct EmailsResponse {
    pub emails: Vec<Email>,
    pub count: usize,
}

#[derive(Deserialize, Clone)]
pub struct Email {
    #[allow(dead_code)]
    pub id: String,
    pub from: String,
    #[allow(dead_code)]
    pub to: String,
    pub subject: String,
    pub body: String,
    #[serde(rename = "receivedAt")]
    pub received_at: u64,
}

impl Client {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.trim_end_matches('/').to_string(),
            http: HttpClient::new(),
        }
    }

    pub async fn create_alias(&self, prefix: Option<&str>) -> Result<AliasResponse> {
        let url = match prefix {
            Some(p) => format!("{}/api/alias?prefix={}", self.base, p),
            None => format!("{}/api/alias", self.base),
        };

        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("Failed to create alias: {}", resp.status()));
        }

        resp.json().await.map_err(Into::into)
    }

    pub async fn get_emails(&self, alias: &str, signature: &str) -> Result<EmailsResponse> {
        let url = format!("{}/api/emails/{}/{}", self.base, alias, signature);

        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("Failed to get emails: {}", resp.status()));
        }

        resp.json().await.map_err(Into::into)
    }

    pub async fn delete_alias(&self, alias: &str, signature: &str) -> Result<()> {
        let url = format!("{}/api/delete/{}/{}", self.base, alias, signature);

        let resp = self.http.delete(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("Failed to delete alias: {}", resp.status()));
        }

        Ok(())
    }
}
