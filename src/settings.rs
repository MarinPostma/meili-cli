use structopt::StructOpt;
use serde::Serialize;
use serde_json::Value;
use anyhow::Result;

use crate::Context;

// find a way to get symonyms
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
    async fn exec(&self, context: &Context, index: &str) -> Result<Value> {
        let slug = format!("indexes/{}/settings", index);
        let response = context.post(&slug, self).await?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "update indexes settings")]
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
    pub async fn exec(&self, context: &Context, index: &str) -> Result<Value> {
        use Settings::*;

        match self {
            Reset => reset(context, index).await,
            List => list(context, index).await,
            Update { settings, ..} => settings.exec(context, index).await,
        }
    }
}

async fn list(context: &Context, index: &str) -> Result<Value> {
    let slug = format!("indexes/{}/settings", index);
    let response = context.get(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn reset(context: &Context, index: &str) -> Result<Value> {
    let slug = format!("indexes/{}/settings", index);
    let _response = context.delete(&slug).await?;
    Ok(Value::String(String::from("ok")))
}
