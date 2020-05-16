use structopt::StructOpt;
use serde::Serialize;
use anyhow::Result;
use serde_json::{Value, json};

#[derive(Debug, StructOpt, Serialize)]
#[structopt(about = "create, delete, update and list indexes")]
pub enum Index {
    Create {
        #[structopt(name = "index_uuid")]
        index_uid: String,
        #[structopt(long)]
        primary_key: Option<String>,
    },
    Delete {
        #[structopt(name = "index uuid")]
        uid: String,
    },
    Get {
        #[structopt(name = "index uid", required_unless("all"))]
        uid: Option<String>,
        #[structopt(long, short)]
        all: bool,
    }
}

impl Index {
    pub async fn exec(&self, addr: &str) -> Result<Value> {
        use Index::*;

        match self {
            Create { index_uid, primary_key } => create_index(&addr, &index_uid, primary_key.as_deref()).await,
            Delete { uid } => delete_index(&addr, uid).await,
            Get { uid, all } => {
                if *all {
                    get_all_indices(&addr).await
                } else {  
                    get_index(&addr, &uid.as_deref().unwrap()).await
                }
            }
        }
    }
}

async fn get_index(addr: &str, uid: &str) -> Result<Value> {
    let url = format!("{}/indexes/{}", addr, uid);
    let response = reqwest::get(&url)
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&response)?)
}

async fn get_all_indices(addr: &str) -> Result<Value> {
    let url = format!("{}/indexes", addr);
    let response = reqwest::get(&url)
        .await?
        .text()
        .await?;
    println!("{:?}", response);
     Ok(serde_json::from_str(&response)?)
}

async fn delete_index(addr: &str, uid: &str) -> Result<Value> {
    let url = format!("{}/indexes/{}", addr, uid);
    let client = reqwest::Client::new();
    let _response = client
        .delete(&url)
        .send()
        .await?
        .text()
        .await?;
    Ok(Value::String(String::from("Ok")))
}

async fn create_index(addr: &str, uid: &str, primary_key: Option<&str>) -> Result<Value> {
    let url = format!("{}/indexes", addr);
    let body = json!({
        "uid": uid,
        "primaryKey": primary_key
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    Ok(serde_json::from_str(&response)?)
}
