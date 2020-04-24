# weather-cli
Get the weather data for any location using OpenWeatherMap in your terminal!

# Requirements
* Rust - v1.41 or higher
* Cargo - v1.41 or higher

# Install
Build the binary inside the root of the project
```
cargo build --release
```
Then place the binary file **target/release/weather-cli** where your local binaries are stored.

# Usage
To query the data for a single location you need to give it the city and country code
```
$ weather-cli -l "London,UK"
```
You can also add a default location to the config file.
```
[default]
city="London"
country="UK"
```
The applciation will use the default location if no argument is given. The config file is stored in **$HOME/.config/weathercli/wcliconf.toml** on Linux.

NOTE: When running the application for the first time you will be asked for an OpenWeatherMap API key. This is necessary for it to be able to query OpenWeatherMap for weather data. This key will be stored locally in the config file.

# License
Licensed under the MIT license. See [LICENSE](LICENSE) for details.
