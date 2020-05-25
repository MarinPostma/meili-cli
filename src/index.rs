use structopt::StructOpt;
use serde::Serialize;
use anyhow::Result;
use serde_json::{Value, json};

use crate::context::Context;

#[derive(Debug, StructOpt, Serialize)]
#[structopt(about = "create, delete, update and list indexes")]
pub enum Index {
    Create {
        #[structopt(name = "index_uid")]
        index_uid: String,
        #[structopt(long)]
        primary_key: Option<String>,
    },
    Delete {
        #[structopt(name = "index_uid")]
        uid: String,
    },
    Get {
        #[structopt(name = "index_uid", required_unless("all"))]
        uid: Option<String>,
        #[structopt(long, short)]
        all: bool,
    },
    List,
}

impl Index {
    pub async fn exec(&self, context: &Context) -> Result<Value> {
        use Index::*;

        match self {
            List => list_indices(&context).await,
            Create { index_uid, primary_key } => create_index(&context, index_uid, primary_key.as_deref()).await,
            Delete { uid } => delete_index(&context, uid).await,
            Get { uid, all } => {
                if *all {
                    get_all_indices(&context).await
                } else {
                    get_index(&context, &uid.as_deref().unwrap()).await
                }
            }
        }
    }
}

async fn list_indices(context: &Context) -> Result<Value> {
    let slug = format!("{}/indexes", &context.host);
    let response = context.get(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn get_index(context: &Context, uid: &str) -> Result<Value> {
    let slug = format!("indexes/{}", uid);
    let response = context.get(&slug).await?;
    Ok(serde_json::from_str(&response)?)
}

async fn get_all_indices(context: &Context) -> Result<Value> {
    let response = context.get("indexes").await?;
    Ok(serde_json::from_str(&response)?)
}

async fn delete_index(context: &Context, uid: &str) -> Result<Value> {
    let slug = format!("indexes/{}", uid);
    let _response = context.delete(&slug).await?;
    Ok(Value::String(String::from("ok")))
}

async fn create_index(context: &Context, uid: &str, primary_key: Option<&str>) -> Result<Value> {
    let payload = json!({
        "uid": uid,
        "primaryKey": primary_key
    });
    let response = context.post("/indexes", &payload).await?;
    Ok(serde_json::from_str(&response)?)
}
