use anyhow::Result;
use reqwest::Body;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Context {
    #[structopt(long, short, default_value = "http://localhost:7700")]
    pub host: String,
    #[structopt(long, short, env = "MEILI_CLI_KEY")]
    pub key: Option<String>,
}

impl Context {
    pub async fn get(&self, slug: &str) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .get(&url);
        if let Some(ref key) = self.key {
            client = client.header("X-Meili-API-Key", key);
        }
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn post<T: Into<Body>>(&self, slug: &str, payload: T) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/json")
            .body(payload);
        if let Some(ref key) = self.key {
            client = client.header("X-Meili-API-Key", key);
        }
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn put<T: Into<Body>>(&self, slug: &str, payload: T) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .put(&url)
            .header("Content-Type", "application/json")
            .body(payload);
        if let Some(ref key) = self.key {
            client = client.header("X-Meili-API-Key", key);
        }
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn delete(&self, slug: &str) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .delete(&url);
        if let Some(ref key) = self.key {
            client = client.header("X-Meili-API-Key", key);
        }
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}
