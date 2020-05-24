use structopt::StructOpt;

use command::Command;

mod index;
mod search;
mod documents;
mod command;
mod settings;

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    match command.exec().await {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}
