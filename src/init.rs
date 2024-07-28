use std::fs;
use std::env;
use std::process;
// use reqwest;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InitArgs {
	#[arg(short, long, default_value="haych_project")]
	project_directory: String,
	#[arg(short, long, default_value="templates")]
	template_directory: String,
	#[arg(short, long, default_value="static")]
	static_directory: String,
	// #[arg(long, default_value="npm")]
	// package_manager: String,
}


pub fn init_project(args: InitArgs) {

	let create_project_dir_res = fs::create_dir(&args.project_directory);
	let create_templates_dir_res = fs::create_dir(
		format!("{}/{}", 
			&args.project_directory, args.template_directory)
		);
	env::set_current_dir(&args.project_directory);
	let create_static_dir_res = fs::create_dir( 
			args.static_directory
		);
	fs::File::create(
		"templates/header.html"
		);

	let index_write_res = fs::write(
		"index.html",
		include_str!("default_index.html")
		);

	let config_write_res = fs::write(
		".haych.toml",
		include_str!("default_config.toml")
		);
}