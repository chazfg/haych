use std::fs;
use tera::Context;
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BuildArgs {
	#[arg(long, default_value="layout.html")]
	source_html: String,
	#[arg(long, default_value="templates/")]
	templates: String,
	#[arg(long, default_value="scripts/")]
	scripts: String,
	#[arg(short, long, default_value="dist/")]
	dist: String,
}

pub fn build_application(args: BuildArgs) {
	let mut app_tree = crate::handle_tree::get_tree();
	app_tree.add_template_file("index.html", None);
	let mut context = Context::new();
	let built_html = app_tree.render("index.html", &context).unwrap();

	let create_dist_dir = fs::create_dir("dist/");
	fs::write("dist/index.html", built_html);

}

pub fn build_command() -> Command {
	Command::new("build")
		.long_about("runs the build application")
		.aliases(["b"])

}