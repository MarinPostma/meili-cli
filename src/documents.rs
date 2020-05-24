use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use structopt::StructOpt;

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
        many: Vec<usize>,
        #[structopt(short, long)]
        all: bool,
        #[structopt(name = "index uid", required_unless("all"), required_unless("many"))]
        docid: usize,
    }
}

impl Documents {
    pub async fn exec(&self, addr: &str, index: &str) -> Result<Value> {
        use Documents::*;

        match self {
            Add { replace, path, .. } => add_documents(&addr, index, path, *replace).await,
            Delete { all, docid, many } => {
                if *all {
                    delete_all(&addr, index).await
                } else if !many.is_empty() {
                    delete_many(&addr, index, &many).await
                } else {
                    delete_one(&addr, index, *docid).await
                }
            }
        }
    }
}

async fn delete_many(_addr: &str, _index: &str, _docids: &[usize]) -> Result<Value> {
    unimplemented!("deleting many documents is not yet available...")
}

async fn delete_one(addr: &str, index: &str, docid: usize) -> Result<Value> {
   let url = format!("{}/indexes/{}/documents/{}", addr, index, docid);
   let client = reqwest::Client::new();
   let response = client
       .delete(&url)
       .send()
       .await?
       .text()
       .await?;
    Ok(serde_json::from_str(&response)?)
}

async fn delete_all(addr: &str, index: &str) -> Result<Value> {
    let url = format!("{}/indexes/{}/documents", addr, index);
    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .send()
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&response)?)
}

async fn add_documents(addr: &str, index: &str, path: &PathBuf, replace: bool) -> Result<Value> {
    let url = format!("{}/indexes/{}/documents", addr, index);
    let client = reqwest::Client::new();
    let client = if replace {
        client.post(&url)
    } else {
        client.put(&url)
    };
    let json_file = File::open(path)?;
    let payload = serde_json::from_reader::<_, Value>(json_file)?; 
    let response = client.json(&payload)
        .send()
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&response)?)
}
