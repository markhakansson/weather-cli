use dirs;
use std::io;
use std::path::PathBuf;
use std::{fs::File, io::prelude::*};

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    api: API,
    default: Default
}

#[derive(Deserialize)]
struct Default {
    city: String,
    country: String
}

#[derive(Deserialize)]
struct API {
    app_id: String
}


pub fn read_config() -> Result<String, io::Error> {
    let conf_path = dirs::home_dir().unwrap().join(".wcliconf");

    let mut file = File::open(conf_path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;

    Ok(result)
}

pub fn init() {
    let config = read_config().unwrap();
    //let content = parse_config(&config);
}