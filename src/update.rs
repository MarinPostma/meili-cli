use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;

use crate::Context;

#[derive(StructOpt, Debug, Serialize)]
#[structopt(about = "get update status")]
pub struct Update {
    #[structopt(
        name = "update id",
        help("The id of the update")
    )]
    id: Option<usize>
}

impl Update {
    pub async fn exec(&self, context: &Context, index: &str) -> Result<Value> {
        let url = format!("/indexes/{}/updates/{}",
            index,
            self.id.map(|id| id.to_string()).unwrap_or_default());
        let response = context.get(&url).await?;
        Ok(serde_json::from_str(&response)?)
    }
}
