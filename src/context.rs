use serde::Serialize;
use structopt::StructOpt;
use anyhow::Result;

#[derive(StructOpt, Debug)]
pub struct Context {
    #[structopt(long, short, default_value = "http://localhost:7700")]
    pub host: String,
}

impl Context {
    pub async fn get(&self, slug: &str) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .get(&url);
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn post<S: Serialize>(&self, slug: &str, payload: &S) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .post(&url)
            .json(payload);
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn put<S: Serialize>(&self, slug: &str, payload: &S) -> Result<String> {
        let url = format!("{}/{}", self.host, slug);
        let mut client = reqwest::Client::new()
            .put(&url)
            .json(payload);
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
        let response = client
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}
