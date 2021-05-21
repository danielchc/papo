use std::path::Path;
use std::io::BufWriter;
use std::fs;
use crate::arguments::Init;
use crate::papo_json::Config;
use crate::logger::{print_warning, print_success, print_info};


pub fn init_project(conf: Init) {
	let path= Path::new(&conf.directory);
	let file= format!("{}/papo.json",conf.directory);
	let mut config:Config=Config{ name: "".to_string(), src: "".to_string() };

	if Path::new(&file).exists(){
		print_warning("Project already initialized");
		return;
	}

	if !path.is_dir() {
		fs::create_dir(&conf.directory).expect("Unable to create directory");
	}
	print_info(&*format!("Creating {}...", file));

	if conf.src {
		fs::create_dir(&(format!("{}/src",conf.directory))).ok();
		config.src= "./src".to_string();
		print_info(&*format!("Creating {}...", "./src"));
	}



	config.name="papo-sample".to_string();
	if conf.name.is_some(){
		config.name=conf.name.unwrap();
	}

	let f = fs::File::create(file).expect("Unable to create file");
	let bw = BufWriter::new(f);
	serde_json::to_writer_pretty(bw, &config).expect("Failed writing :(");
	print_success(&*format!("Project {} created", config.name));
}