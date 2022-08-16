mod args;
mod modules;

use clap::Parser;
use args::{Opts,SubCommand};
use modules::start::start_pod;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
	let opts: Opts = Opts::parse();
	match opts.subcmd {
		SubCommand::Start(_t) => {
            start_pod(_t).await;
        }
	}
}