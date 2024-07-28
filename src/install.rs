use std::fs;
use std::env;
use std::process;
use toml::Table;
// use reqwest;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InstallArgs {
	package: String,
}


pub fn install_package(args: InstallArgs) {

	let current_directory = env::current_dir().unwrap();

	let packages =  include_str!("default_paths.toml").parse::<Table>().unwrap();

	if let Some(p) = packages.get(&args.package) {
		let package_text = reqwest::blocking::get(p.get("link").unwrap().as_str().unwrap())
			.unwrap()
			.text()
			.unwrap();

		fs::write(
			format!("static/{}", args.package),
			package_text
			);
		
		
	} else {
		println!("could not find");
	}
	


}