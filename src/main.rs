mod arguments;
mod init_project;
mod types;
mod papo_json;
mod logger;

use std::{env};
use std::path::Path;

use clap::{Clap};
use std::process::exit;
use crate::arguments::{Opts,SubCommand};
use crate::types::{Jdk};
use crate::init_project::init_project;
use crate::logger::print_error;


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
		print_error("ERROR: Java not found. Please set the JAVA_HOME variable in your environment or use the option --jdk to match the location of your Java installation");
		exit(1);
	}

	match opts.subcmd {
		SubCommand::Init(t) => init_project(t),
	}
}

