use structopt::StructOpt;
use serde::Serialize;
use serde_json::Value;
use anyhow::Result;

// find a way to het symonyms
#[derive(Debug, StructOpt, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[structopt(
        long,
        value_names(&["rule"]),
        help("Fields in which to search for matching query words sorted by order of importance"),
    )]
    ranking_rules: Option<Vec<String>>,
    #[structopt(
        long,
        value_names(&["word"]),
        help("List of ranking rules sorted by order of importance"),
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_words: Option<Vec<String>>,
    #[structopt(
        long,
        help("Search returns documents with distinct (different) values of the given field"),
        value_names(&["attr"])
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    distinct_attribute: Option<String>,
    #[structopt(
        long,
        help("Fields in which to search for matching query words sorted by order of importance"),
        value_names(&["attr"])
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    searchable_attributes: Option<Vec<String>>,
    #[structopt(
        long,
        help("Fields displayed in the returned documents"),
        value_names(&["attr"])
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    displayed_attributes: Option<Vec<String>>,
    #[structopt(
        long,
        help("Defines whether new fields should be searchable and displayed or not"),
        value_names(&["boolean"])
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_new_fields: Option<bool>,
}

impl SettingsUpdate {
    async fn exec(&self, addr: &str, index: &str) -> Result<Value> {
        let url = format!("{}/indexes/{}/settings", addr, index);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&self)
            .send()
            .await?
            .text()
            .await?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "create, delete, update and list indexes")]
pub enum Settings {
    #[structopt(
        about("Reset all settings for index"),
    )]
    Reset,
    #[structopt(
        about("List all settings for index"),
    )]
    List,
    #[structopt(
        about("Update settings for index"),
    )]
    Update {
        #[structopt(flatten)]
        settings: SettingsUpdate,
    },
}

impl Settings {
    pub async fn exec(&self, addr: &str, index: &str) -> Result<Value> {
        use Settings::*;

        match self {
            Reset => reset(addr, index).await,
            List => list(addr, index).await,
            Update { settings, ..} => settings.exec(addr, index).await,
        }
    }
}

async fn list(addr: &str, index: &str) -> Result<Value> {
    let url = format!("{}/indexes/{}/settings", addr, index);
    let response = reqwest::get(&url)
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&response)?)
}

async fn reset(addr: &str, index: &str) -> Result<Value> {
    let url = format!("{}/indexes/{}/settings", addr, index);
    let client = reqwest::Client::new();
    let _response = client
        .delete(&url)
        .send()
        .await?
        .text()
        .await?;
    Ok(Value::String(String::from("ok")))
}
