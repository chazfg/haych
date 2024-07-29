use toml::Table;
use std::{fs, env, path::PathBuf};

#[derive(Debug)]
pub struct ProjectConfig(Table);

impl ProjectConfig {
	pub fn init(cli_config: &Option<&PathBuf>) -> Result<Self, ()> {

		// should add a check here to see if we're even in a haych project 

		if let Some(config_path) = cli_config {
			if let Ok(dir_config) = fs::read_to_string(config_path) {
				Ok(Self(dir_config.parse::<Table>().unwrap()))
			} else {
				Err(())
			}
		} else {
			if let Ok(dir_config) = fs::read_to_string(".haych.toml") {
				Ok(Self(dir_config.parse::<Table>().unwrap()))
			} else {
				Err(())
			}
		}
	}
}