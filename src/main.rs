pub mod config;
mod build_cmd;
mod init;
mod install;
mod watch;
pub mod handle_tree;
use watch::{WatchArgs, watch_command, watch_files};
use init::{InitArgs, init_project};
use build_cmd::{BuildArgs, build_command, build_application};
use install::{InstallArgs, install_package};
use clap::{Parser, Subcommand, FromArgMatches, Command, Args, ArgMatches};
use clap::error::{Error, ErrorKind};
extern crate minify;
use minify::html::minify;
use std::fs;
use toml::Table;

#[derive(Debug)]
enum CliSub {
    Build(BuildArgs),
    Init(InitArgs),
    Install(InstallArgs),
    Watch(WatchArgs)
}


#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: CliSub,
}
fn main() {
    let args = Cli::parse();
	
    match args.subcommand {
        CliSub::Build(args) => build_application(args),
        CliSub::Init(args) => init_project(args),
        CliSub::Install(args) => install_package(args),
        CliSub::Watch(args) => watch_files(args),
        _ => ()
    }
    
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("build", args)) => Ok(Self::Build(BuildArgs::from_arg_matches(args)?)),
            Some(("init", args)) => Ok(Self::Init(InitArgs::from_arg_matches(args)?)),
            Some(("install", args)) => Ok(Self::Install(InstallArgs::from_arg_matches(args)?)),
            Some(("watch", args)) => Ok(Self::Watch(WatchArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `build` `install` `init`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `build` `install` `init`",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("build", args)) => *self = Self::Build(BuildArgs::from_arg_matches(args)?),
            Some(("init", args)) => *self = Self::Init(InitArgs::from_arg_matches(args)?),
            Some(("install", args)) => *self = Self::Install(InstallArgs::from_arg_matches(args)?),
            Some(("watch", args)) => *self = Self::Watch(WatchArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `build` `install` `init`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for CliSub {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(BuildArgs::augment_args(build_command()))
            .subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(InstallArgs::augment_args(Command::new("install")))
            .subcommand(WatchArgs::augment_args(watch_command()))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(BuildArgs::augment_args(Command::new("build")))
            .subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(InstallArgs::augment_args(Command::new("install")))
            .subcommand(WatchArgs::augment_args(watch_command()))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "build" | "init" | "install")
    }
}
