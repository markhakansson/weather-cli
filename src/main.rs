extern crate dirs;

use structopt::StructOpt;

mod confighandler;

const ZERO_CELSIUS_IN_KELVIN: f32 = 273.15;
const HTTP_OK: u32 = 200;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    /// The city to get weather data from. Use the following format: "City,Country code", e.g. "London,UK" within quotation marks.
    location: Option<String>,
    #[structopt(short = "h", long = "hometown")]
    /// Set default hometown. Will be used if no location is specified.
    hometown: Option<String>
}

fn main() {
    confighandler::init();
    let args = Cli::from_args();
    println!("{:?}", args);
}