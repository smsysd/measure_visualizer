use std::{io::{Error, Read, ErrorKind}, fs::File};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::frames::control::Deltas;

pub const CONFIG_PATH: &str = "./config.toml";

#[derive(Resource, Deserialize, Serialize)]
pub struct Config {
	pub default_deltas: Deltas
}

pub fn load_config() -> Result<Config, Error> {
	let mut config_file = File::open(CONFIG_PATH)?;
	let mut config_raw = Vec::new();
	let config_raw_len = config_file.read_to_end(&mut config_raw)?;
	match toml::from_slice(&config_raw[..config_raw_len]) {
		Ok(c) => Ok(c),
		Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string()))
	}
}