use structopt::StructOpt;
use serde_json::Value;
use anyhow::Result;

use crate::index::Index;
use crate::search::Search;
use crate::documents::Documents;
use crate::settings::Settings;

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
    },
    Settings {
        #[structopt(short, long)]
        index: String,
        #[structopt(subcommand)]
        settings: Settings,
    }
}

#[derive(StructOpt, Debug)]
pub struct Command {
    #[structopt(long, short, default_value = "http://localhost:7700")]
    host: String,
    #[structopt(subcommand)]
    query: Query,
}

impl Command {
    pub async fn exec(&self) -> Result<Value> {
        use Query::*;

        match self.query {
            Search { ref search, ref index, .. } => search.exec(&self.host, index).await,
            Index { ref index } => index.exec(&self.host).await,
            Documents { ref documents, ref index } => documents.exec(&self.host, index).await,
            Settings { ref index, ref settings, .. } => settings.exec(&self.host, index).await,
        }
    }
}
