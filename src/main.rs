extern crate dirs;

use structopt::StructOpt;

mod confighandler;
mod owm;

use confighandler as config;

const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    /// The city to get weather data from. Use the following format: "City,Country code", e.g. "London,UK" within quotation marks.
    location: Option<String>,
    #[structopt(short = "h", long = "hometown")]
    /// Set default hometown. Will be used if no location is specified.
    hometown: Option<String>,
}

fn main() {
    let config = config::read_config().unwrap();
    let app_id = config.api.app_id.as_str();

    let args = Cli::from_args();

    let location: String;
    
    match args.location {
        None => {
            let default = config.default.unwrap();
            let mut res = default.city;
            res.push_str(",");
            res.push_str(default.country.as_str());
            location = res;
        },
        Some(l) => location = l,
    };

    let url = "https://api.openweathermap.org/data/2.5/weather";
    let query = ["q=", location.as_str(), "&appid=", app_id].concat().to_string();

    let data = owm::get_current_weather(url.to_string(), query).unwrap();
    let weather = data.weather.unwrap().pop().unwrap();
    let main = data.main.unwrap();

    println!("Current weather in {} is: {}", data.name.unwrap(), weather.description.unwrap());
    println!("Temp: {:.1}", main.temp.unwrap() - ZERO_CELSIUS_IN_KELVIN);
}
