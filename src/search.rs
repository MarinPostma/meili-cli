use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize)]
pub struct Search {
    #[structopt(name = "query")]
    q: String,
    #[structopt(long)]
    offset: Option<usize>,
    #[structopt(long)]
    limit: Option<usize>,
    #[structopt(long)]
    attributes_to_retrieve: Option<Vec<String>>,
    #[structopt(long)]
    attributes_to_crop: Option<Vec<String>>,
    #[structopt(long)]
    crop_length: Option<usize>,
    #[structopt(long)]
    attributes_to_highlight: Option<Vec<String>>,
    #[structopt(long)]
    filters: Option<String>,
    #[structopt(long)]
    matches: Option<bool>,
}

impl Search {
    pub async fn exec(&self, addr: &str, index: &str) -> Result<Value> {
        let url_params = serde_url_params::to_string(&self).unwrap();
        let url = format!("{}/indexes/{}/search?{}", addr, index, url_params);
        let response = reqwest::get(&url)
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&response)?)
    }
}
