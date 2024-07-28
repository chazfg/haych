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
}