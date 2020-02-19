use structopt::StructOpt;
use anyhow::{anyhow, Result};
use crate::confighandler as config;
use crate::owm;

const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    /// The city to get weather data from. Use the following format: "City,Country code", e.g. "London,UK" within quotation marks.
    location: Option<String>
}

pub fn cli() -> Result<()> {
    let config = config::read_config()?;
    let app_id = config.api.app_id.as_str();

    let args = Cli::from_args();

    let location: String;

    match args.location {
        Some(l) => location = l,
        None => {
            match config.default {
                Some(default) => {
                    let mut res = default.city;
                    res.push_str(",");
                    res.push_str(default.country.as_str());
                    location = res;
                },
                None => return Err(anyhow!("Could not query data. No location argument given and default location is not set!"))
            }
        }
    };

    let query = ["q=", location.as_str(), "&appid=", app_id].concat().to_string();
    let data = owm::get_current_weather(query)?;

    print_weather_data(data);

    Ok(())
}

// should not throw an error if data is empty, should just print n/a or something
fn print_weather_data(data: owm::CurrentWeather) {
    let weather = data.weather.unwrap().pop().unwrap();
    let main = data.main.unwrap();

    println!("Current weather in {} is: {}", data.name.unwrap(), weather.description.unwrap());
    println!("Temp: {:.1}", main.temp.unwrap() - ZERO_CELSIUS_IN_KELVIN);
}