use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;

use crate::Context;

#[derive(StructOpt, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[structopt(about = "perform search")]
pub struct Search {
    #[structopt(
        name = "query",
        help("query string")
    )]
    q: String,
    #[structopt(
        long,
        help("Number of documents to skip"),
    )]
    offset: Option<usize>,
    #[structopt(
        long,
        help("Maximum number of documents returned"),
    )]
    limit: Option<usize>,
    #[structopt(
        long,
        help("Attributes to display in the returned documents"),
    )]
    attributes_to_retrieve: Option<Vec<String>>,
    #[structopt(
        long,
        help("Attributes whose values have to be cropped"),
    )]
    attributes_to_crop: Option<Vec<String>>,
    #[structopt(
        long,
        help("Length used to crop field values"),
    )]
    crop_length: Option<usize>,
    #[structopt(
        long,
        help("Attributes whose values will contain highlighted matching terms")
    )]
    attributes_to_highlight: Option<Vec<String>>,
    #[structopt(
        long,
        help("Filter queries by an attribute value"),
    )]
    filters: Option<String>,
    #[structopt(
        long,
        help("Defines whether an object that contains information about the matches should be returned or not"),
    )]
    matches: Option<bool>,
    #[structopt(
        long,
    )]
    facet_filters: Option<String>,
    #[structopt(
        long,
    )]
    facets_distribution: Option<String>,
}

impl Search {
    pub async fn exec(&self, context: &Context, index: &str) -> Result<Value> {
        let url_params = serde_url_params::to_string(&self).unwrap();
        let slug = format!("indexes/{}/search?{}", index, url_params);
        let response = context.get(&slug).await?;
        Ok(serde_json::from_str(&response)?)
    }
}
