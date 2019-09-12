use quicli::prelude::*;
use structopt::StructOpt;

//use std::fs;
//use std::io::prelude::*;

use reqwest::Url;
use std::fmt;
//use serde_json;

use std::error::Error;

const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;
const STATUS_OK: u32 = 200;

// Error messages
#[derive(Debug)]
enum CliError {
    UrlError,
    JsonError,
}

impl Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"temporary error message")
    }
}

// Basic struct for the program and the different options
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "location", short = "l", default_value = "London,UK")]
    /// Location to check weather data for. Parameter is "City,Country Code". E.g. "London,UK".
    location: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(Clone,Debug,Deserialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Clone,Debug,Deserialize)]
struct Main {
    temp: f32,
    pressure: f32,
    humidity: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Clone,Debug,Deserialize)]
struct OpenWeatherMapJson {
    weather: Vec<Weather>,
    main: Main,
    name: String,
    cod: u32,
}

/*
fn write_weather_data_to_file(city: String, country: String) { 
    let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("~/.weatherclidata");
}
*/

/// Queries OpenWeatherMap and returns a JSON with the weather data for that query.
fn get_openweathermap_json(url: String, query: String) -> Result<OpenWeatherMapJson,CliError> {
    let mut url = Url::parse(&url).unwrap();
    url.set_query(Some(&query));
    let json: OpenWeatherMapJson = reqwest::get(url).unwrap().json().unwrap();
    Ok(json)
}

/// Prints the weather information to the console from the OWM JSON.
fn print_weather_from_json(json: OpenWeatherMapJson) -> Result<OpenWeatherMapJson,CliError>{
    if json.cod != STATUS_OK {
        //println!("Error {}. Could not retrieve the weather data for the location.", json.cod);
        Err(CliError::JsonError)
    } else {
        let mut w: Vec<Weather> = json.weather.clone();
        let weather: Weather = w.pop().unwrap();
        println!("Current weather in {} is: {}", json.name, weather.description);
        println!("Temp: {:.1}", json.main.temp - ZERO_CELSIUS_IN_KELVIN);
        Ok(json)
    }
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("weather-cli")?;

    let location: String  = "q=".to_owned() + &args.location;
    let app_id: String = "&APPID=a9696b1d40f6012ee309ebf8b10c8b30".to_string();

    let query: String = location.to_owned() + &app_id.to_owned(); 
    let url: String = "https://api.openweathermap.org/data/2.5/weather".to_string();

    let json = get_openweathermap_json(url, query).unwrap();

    print_weather_from_json(json);
    
    Ok(())
}


