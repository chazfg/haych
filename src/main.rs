pub mod config;
mod build_cmd;
mod init;
mod install;
mod watch;
pub mod handle_tree;
use watch::{WatchArgs, watch_command, watch_files};
use init::{init_project, init_command};
use build_cmd::{build_command, build_application};
use install::{InstallArgs, install_package};
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches, arg, command, value_parser};
use clap::error::{Error, ErrorKind};
extern crate minify;
use minify::html::minify;
use std::fs;
use std::path::PathBuf;
use toml::Table;


fn main() {

    let matches = command!()
        .arg(
            arg!(-c --config <CONFIG> "Set path to config file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
            )
        .subcommand(build_command())
        .subcommand(watch_command())
        .subcommand(init_command())
        .get_matches();


	let project_config = config::ProjectConfig::init(&matches.get_one::<PathBuf>("config"));

    match matches.subcommand() {

        Some(("watch", args)) => watch_files(args, project_config.expect("Could not find config file")),
        Some(("init", args)) => init_project(args),
        _ => ()

    };


    // let args = Cli::parse();
    // 
    // match args.subcommand {
        // CliSub::Build(args) => build_application(args),
        // CliSub::Init(args) => init_project(args).unwrap(),
        // CliSub::Install(args) => install_package(args),
        // CliSub::Watch(args) => watch_files(args, project_config),
        // _ => ()
    // }
    
}
