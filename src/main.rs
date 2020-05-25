use structopt::StructOpt;

use command::Command;

pub use context::Context;

mod index;
mod search;
mod documents;
mod command;
mod settings;
mod context;

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    match command.exec().await {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}
