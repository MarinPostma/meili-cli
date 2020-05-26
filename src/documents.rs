use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;

use crate::Context;

#[derive(Debug, StructOpt, Serialize)]
#[structopt(about = "create, delete, update and list documents")]
pub enum Documents {
    Add {
        #[structopt(short, long, conflicts_with("update"))]
        replace: bool,
        #[structopt(short, long, required_unless("replace"))]
        update: bool,
        #[structopt(name = "path", parse(from_os_str))]
        path: PathBuf,
    },
    Delete {
        #[structopt(short, long, conflicts_with("all"))]
        multiple: Vec<usize>,
        #[structopt(short, long)]
        all: bool,
        #[structopt(name = "index uid", required_unless("all"), required_unless("many"))]
        docid: usize,
    },
    List,
}

impl Documents {
    pub async fn exec(&self, context: &Context, index: &str) -> Result<Value> {
        use Documents::*;

        match self {
            Add { replace, path, .. } => add_documents(context, index, path, *replace).await,
            Delete { all, docid, multiple } => {
                if *all {
                    delete_all(context, index).await
                } else if !multiple.is_empty() {
                    delete_many(context, index, multiple).await
                } else {
                    delete_one(context, index, *docid).await
                }
            },
            List => list_documents(context, index).await,
        }
    }
}

async fn list_documents(context: &Context, index: &str) -> Result<Value> {
    let slug = format!("indexes/{}/documents", index);
    let response = context.get(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn delete_many(_context: &Context, _index: &str, _docids: &[usize]) -> Result<Value> {
    unimplemented!("deleting many documents is not yet available...")
}

async fn delete_one(context: &Context, index: &str, docid: usize) -> Result<Value> {
   let slug = format!("indexes/{}/documents/{}", index, docid);
   let response = context.delete(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn delete_all(context: &Context, index: &str) -> Result<Value> {
    let slug = format!("indexes/{}/documents", index);
    let response = context.delete(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn add_documents(context: &Context, index: &str, path: &PathBuf, replace: bool) -> Result<Value> {
    let url = format!("indexes/{}/documents", index);
    let payload = read_to_string(&path)?;
    let response = if replace {
        context.post(&url, payload).await?
    } else {
        context.put(&url, payload).await?
    };
    Ok(serde_json::from_str(&response)?)
}
