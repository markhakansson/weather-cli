use dirs;
use std::io;
use std::path::PathBuf;
use std::{fs::File, io::prelude::*};

use serde::Deserialize;
use toml;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
enum ConfError {
    
}

#[derive(Deserialize, Debug)]
pub struct Config {
    api: API,
    default: Default
}

#[derive(Deserialize, Debug)]
pub struct Default {
    city: String,
    country: String
}

#[derive(Deserialize, Debug)]
pub struct API {
    app_id: String
}

fn check_config_path() {}

/// Reads 
pub fn read_config() -> Result<Config> {
    let conf_path: PathBuf;

     match dirs::config_dir() {
        Some(dir) => {
            conf_path = dir
                        .join("weathercli")
                        .join("wcliconf.toml")
        },
        None => return Err(anyhow!("Could not find the config directory in your system!"))
    }

    println!("{:#?}", conf_path);

    let mut file = File::open(conf_path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;

    let conf: Config = toml::from_str(result.as_str())?;

    Ok(conf)
}

pub fn init() {
    let config = read_config().unwrap();    
    println!("{:#?}", config);
}