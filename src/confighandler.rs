use dirs;
use std::io;
use std::path::PathBuf;
use std::{fs, fs::File, io::prelude::*};

use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    api: API,
    default: Option<Default>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Default {
    city: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct API {
    app_id: String,
}

/// Initializes a new basic config file with an OpenWeatherMap API key. 
fn init_new_config(config_path: PathBuf) -> Result<Config> {
    let mut input = String::new();
    println!("This program requires an OpenWeatherMap API key to query the weather data. Please insert your API key: ");
    io::stdin().read_line(&mut input)?;
    input.pop();

    let new_config: Config = Config {
        api: { API { app_id: input } },
        default: None,
    };

    let mut file = File::create(config_path)?;
    let toml = toml::to_string(&new_config)?;
    file.write_all(toml.as_bytes())?;

    Ok(new_config)
}

/// Reads the config file. If it doesn't exist it will create a new.
pub fn read_config() -> Result<Config> {
    let conf_path: PathBuf;

    match dirs::config_dir() {
        Some(dir) => {
            conf_path = dir.join("weathercli").join("wcliconf.toml");
        }
        None => {
            return Err(anyhow!(
                "Could not find the config directory in your system!"
            ))
        }
    }

    if !conf_path.exists() {
        init_new_config(conf_path)
    } else {
        let mut file = File::open(conf_path)?;
        let mut result = String::new();
        file.read_to_string(&mut result)?;

        let conf: Config = toml::from_str(result.as_str())?;

        Ok(conf)
    }
}

pub fn init() {
    let config = read_config().unwrap();
    println!("{:#?}", config);
}
