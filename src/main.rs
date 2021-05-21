use clap::{AppSettings, Clap};
use std::env;
use std::io;
use std::os;
use std::path::PathBuf;


#[derive(Clap)]
#[clap(version = "1.0", author = "danielchc")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
	#[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init(Init),
}

#[derive(Clap)]
struct Init {
    #[clap(default_value = ".")]
	directory: String,
    #[clap(short,long)]
	name: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
	match opts.subcmd {
		SubCommand::Init(t) => init(t),
	}
}

fn init(conf:Init){
	println!("Creating papo config... {}",conf.directory);
}