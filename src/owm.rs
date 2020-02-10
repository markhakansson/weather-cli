#[derive(Clone,Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Clone,Debug)]
struct Main {
    temp: f32,
    pressure: f32,
    humidity: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Clone,Debug)]
struct OpenWeatherMapJson {
    weather: Vec<Weather>,
    main: Main,
    name: String,
    cod: u32,
}