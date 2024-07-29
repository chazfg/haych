use std::{fs, path::PathBuf};
use tera::Context;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches, arg, value_parser};



pub fn build_application() {
	let mut app_tree = crate::handle_tree::get_tree();
	let mut context = Context::new();
	let built_html = app_tree.render("layout.html", &context).unwrap();

	let create_dist_dir = fs::create_dir("dist/");
	fs::write("dist/index.html", built_html)
		.expect("should write");

}

pub fn build_command() -> Command {
	Command::new("build")
                .about("builds application")
                .arg(
                    arg!(-o --output_dir <OUTPUT_DIR> "build directory")
                        .required(false)
                        .value_parser(value_parser!(PathBuf))
                    )
}