use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches, arg, value_parser};
// use reqwest;

pub fn init_command() -> Command {
    Command::new("init")
        .about("initialize project")
        .arg(
        	arg!(-b --backend <BACKEND_FRAMEWORK> "only axum supported")
        		.required(false)
        		.default_value("axum")
        	)
}

pub fn init_project(args: &ArgMatches) {
	create_axum_service();
}

fn create_axum_service() {
	println!("Cargo generating axum template");
	let cmd = std::process::Command::new("cargo")
		.arg("generate")
		.arg("--git")
		.arg("https://github.com/chazfg/haxum_template.git")
		.status();

	println!("finished init with {:?}", cmd);
}
