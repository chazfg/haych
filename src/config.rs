use toml::Table;
use std::{fs, env};

pub struct ProjectConfig(Table);

impl ProjectConfig {

	pub fn init() -> Self {

		if let Ok(dir_config) = fs::read_to_string(".haych.toml") {
			Self(dir_config.parse::<Table>().unwrap())
		} else {
			Self(include_str!("default_config.toml").parse::<Table>().unwrap())
		}

	}

}