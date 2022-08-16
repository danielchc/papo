use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "danielchc")]
// #[clap(setting = AppSettings::ColordHelp)]
pub struct Opts {
	#[clap(short, long, parse(from_occurrences))]
	pub verbose: i32,
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
	Start(Start),
}

#[derive(Parser)]
pub struct Start {
	#[clap(default_value = "debian:stable")]
	pub image: String
}
