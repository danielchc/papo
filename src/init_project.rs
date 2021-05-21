use std::path::Path;
use std::io::BufWriter;
use std::fs;
use crate::arguments::Init;
use crate::papo_json::Config;
use crate::logger::{print_warning, print_success};


pub fn init_project(conf: Init) {

	let path= Path::new(&conf.directory);
	let file= format!("{}/papo.json",conf.directory);
	if Path::new(&file).exists(){
		print_warning("Project already initialized");
		return;
	}

	if !path.is_dir() {
		fs::create_dir(&conf.directory).expect("Unable to create directory");
	}

	if conf.src {
		fs::create_dir(&(format!("{}/src",conf.directory))).ok();
	}


	println!("Creating papo config...");
	let config:Config=Config{ name: "".to_string(), src: "".to_string() };
	let f = fs::File::create(file).expect("Unable to create file");
	let bw = BufWriter::new(f);
	serde_json::to_writer(bw, &config).expect("Failed writing :(");
	print_success(&*format!("New project created in {}", conf.directory));
}