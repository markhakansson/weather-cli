use anyhow::{Result};
use reqwest::{Url, StatusCode};
use serde::{Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentWeather {
    pub coord: Option<Coord>,
    pub weather: Option<Vec<Weather>>,
    pub base: Option<String>,
    pub main: Option<Main>,
    pub wind: Option<Wind>,
    pub clouds: Option<Clouds>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub visisbility: Option<u32>,
    pub dt: Option<u64>,
    pub sys: Option<Sys>,
    pub timezone: Option<f64>,
    pub id: Option<u32>,
    pub name: Option<String>,
    pub cod: Option<u32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Coord {
    pub lon: Option<f32>,
    pub lat: Option<f32>
}

#[derive(Deserialize, Clone,Debug)]
pub struct Weather {
    pub id: Option<u32>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Main {
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub pressure: Option<f32>,
    pub humidity: Option<f32>,
    pub temp_min: Option<f32>,
    pub temp_max: Option<f32>,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Wind {
    pub speed: Option<f32>,
    pub deg: Option<f32>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Clouds {
    pub all: Option<u32>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rain {
    #[serde(rename = "1h")]
    pub one_hour: Option<u32>,
    #[serde(rename = "3h")]
    pub three_hour: Option<u32>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Snow {
    #[serde(rename = "1h")]
    pub one_hour: Option<u32>,
    #[serde(rename = "3h")]
    pub three_hour: Option<u32>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_: Option<u32>,
    pub id: Option<u32>,
    pub message: Option<f32>,
    pub country: Option<u32>,
    pub sunrise: Option<u64>,
    pub sunset: Option<u64>
}


/// Queries OpenWeatherMap and returns a JSON with the weather data for that query.
pub fn get_current_weather(url: String, query: String) -> Result<CurrentWeather> {
    let mut url = Url::parse(&url)?;
    url.set_query(Some(&query));
    let json: CurrentWeather = reqwest::blocking::get(url)?.json()?;
    Ok(json)
}