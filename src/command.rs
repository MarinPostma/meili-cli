use structopt::StructOpt;
use serde_json::Value;
use anyhow::Result;

use crate::index::Index;
use crate::search::Search;
use crate::documents::Documents;

#[derive(StructOpt, Debug)]
#[structopt(about = "Meilisearch command line interface")]
enum Query {
    Search {
        #[structopt(short, long)]
        index: String,
        #[structopt(flatten)]
        search: Search,
    },
    Index {
        #[structopt(subcommand)]
        index: Index,
    },
    Documents {
        #[structopt(short, long)]
        index: String,
        #[structopt(subcommand)]
        documents: Documents,
    }
}

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(name = "address", default_value = "http://localhost:7700")]
    addr: String,
    #[structopt(subcommand)]
    query: Query,
}

impl Command {
    pub async fn exec(&self) -> Result<Value> {
        use Query::*;

        match self.query {
            Search { ref search, ref index, .. } => search.exec(&self.addr, index).await,
            Index { ref index } => index.exec(&self.addr).await,
            Documents { ref documents, ref index } => documents.exec(&self.addr, index).await,
        }
    }
}
