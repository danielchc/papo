use std::{env, fs};
use std::path::Path;
use serde::{Deserialize, Serialize};

use clap::{AppSettings, Clap};
use std::process::exit;
use std::io::BufWriter;

#[derive(Clap)]
#[clap(version = "1.0", author = "danielchc")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
	#[clap(short, long, parse(from_occurrences))]
	verbose: i32,
	#[clap(long)]
	jdkdir: Option<String>,
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
	#[clap(short, long)]
	name: Option<String>,
}

struct Jdk{
	avaliable:bool,
	directory:String
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
	name: String,
	src:String
}


fn check_jdk(jdkdirarg: Option<String>) -> Jdk {
	let mut result:Jdk=Jdk{
		avaliable: false,
		directory: "".to_string()
	};
	let javahome: Option<String>;
	javahome = if jdkdirarg.is_some() {jdkdirarg} else {env::var("JAVA_HOME").ok()};
	if javahome.is_none() { return result; };
	result.directory=javahome.unwrap();
	let java_files=vec![format!("{}/java",result.directory), format!("{}/javac",result.directory)];
	for x in java_files{
		if !Path::new(&x.to_string()).exists() {
			return result;
		}
	}
	result.avaliable=true;
	return result;
}

fn main() {
	let opts: Opts = Opts::parse();
	let jdk_result:Jdk= check_jdk(opts.jdkdir);
	if !jdk_result.avaliable {
		eprintln!("ERROR: Java not found. Please set the JAVA_HOME variable in your environment or use the option --jdk to match the location of your Java installation");
		exit(1);
	}

	match opts.subcmd {
		SubCommand::Init(t) => init(t),
	}
}

fn init(conf: Init) {
	let file= format!("{}/papo.json",conf.directory);
	if Path::new(&file).exists(){
		println!("Project already initialized");
		return;
	}

	println!("Creating papo config...");
	let config:Config=Config{ name: "".to_string(), src: "".to_string() };
	let f = fs::File::create("papo.json").expect("Unable to create file");
	let mut bw = BufWriter::new(f);
	serde_json::to_writer(bw, &config).expect("Failed writing :(");

}