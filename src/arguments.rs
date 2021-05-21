use clap::{AppSettings, Clap};
#[derive(Clap)]
#[clap(version = "1.0", author = "danielchc")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
	#[clap(short, long, parse(from_occurrences))]
	pub verbose: i32,
	#[clap(long)]
	pub jdkdir: Option<String>,
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
	Init(Init),
}

#[derive(Clap)]
pub struct Init {
	#[clap(default_value = "./")]
	pub directory: String,
	#[clap(short = 'f', long="create-src")]
	pub src:bool,
	#[clap(short, long)]
	pub name: Option<String>,
}


