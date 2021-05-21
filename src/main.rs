//
//
//		                                                                                          
//		PPPPPPPPPPPPPPPPP            AAA                PPPPPPPPPPPPPPPPP         OOOOOOOOO     
//		P::::::::::::::::P          A:::A               P::::::::::::::::P      OO:::::::::OO   
//		P::::::PPPPPP:::::P        A:::::A              P::::::PPPPPP:::::P   OO:::::::::::::OO 
//		PP:::::P     P:::::P      A:::::::A             PP:::::P     P:::::P O:::::::OOO:::::::O
//		  P::::P     P:::::P     A:::::::::A              P::::P     P:::::P O::::::O   O::::::O
//		  P::::P     P:::::P    A:::::A:::::A             P::::P     P:::::P O:::::O     O:::::O
//		  P::::PPPPPP:::::P    A:::::A A:::::A            P::::PPPPPP:::::P  O:::::O     O:::::O
//		  P:::::::::::::PP    A:::::A   A:::::A           P:::::::::::::PP   O:::::O     O:::::O
//		  P::::PPPPPPPPP     A:::::A     A:::::A          P::::PPPPPPPPP     O:::::O     O:::::O
//		  P::::P            A:::::AAAAAAAAA:::::A         P::::P             O:::::O     O:::::O
//		  P::::P           A:::::::::::::::::::::A        P::::P             O:::::O     O:::::O
//		  P::::P          A:::::AAAAAAAAAAAAA:::::A       P::::P             O::::::O   O::::::O
//		PP::::::PP       A:::::A             A:::::A    PP::::::PP           O:::::::OOO:::::::O
//		P::::::::P      A:::::A               A:::::A   P::::::::P            OO:::::::::::::OO 
//		P::::::::P     A:::::A                 A:::::A  P::::::::P              OO:::::::::OO   
//		PPPPPPPPPP    AAAAAAA                   AAAAAAA PPPPPPPPPP                OOOOOOOOO     
//		 
//
//
use std::path::Path;
use std::io::BufWriter;
use std::fs;
use crate::arguments::Init;
use crate::papo_json::Config;
use crate::logger::{print_warning, print_success, print_info};


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

