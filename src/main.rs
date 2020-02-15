extern crate dirs;

use structopt::StructOpt;

mod confighandler;
mod owm;

use confighandler as config;

const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;
const HTTP_OK: u32 = 200;

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
    let location = "London,UK";
    let url = "https://api.openweathermap.org/data/2.5/weather";
    let query = ["q=", location, "&appid=", app_id].concat().to_string();

    println!("{:#}, {:#}", url, query);

    let test = owm::get_current_weather(url.to_string(), query).unwrap();

    println!("{:#?}", test);

    let args = Cli::from_args();
    println!("{:?}", args);
}
