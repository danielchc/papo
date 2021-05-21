use ansi_term::Colour;

pub fn print_error(msg:&str) {
	println!("{}",Colour::Red.paint(msg));
}

pub fn print_warning(msg:&str) {
	println!("{}",Colour::Yellow.paint(msg));
}
pub fn print_success(msg:&str) {
	println!("{}",Colour::Green.paint(msg));
}