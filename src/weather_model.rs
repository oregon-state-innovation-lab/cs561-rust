use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherModel {
    #[serde(rename = "coord")]
    pub coord: Coord,

    #[serde(rename = "weather")]
    pub weather: Vec<Weather>,

    #[serde(rename = "base")]
    pub base: String,

    #[serde(rename = "main")]
    pub main: Main,

    #[serde(rename = "visibility")]
    pub visibility: i64,

    #[serde(rename = "wind")]
    pub wind: Wind,

    #[serde(rename = "clouds")]
    pub clouds: Clouds,

    #[serde(rename = "dt")]
    pub dt: i64,

    #[serde(rename = "sys")]
    pub sys: Sys,

    #[serde(rename = "timezone")]
    pub timezone: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "cod")]
    pub cod: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    #[serde(rename = "all")]
    pub all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    #[serde(rename = "lon")]
    pub lon: f64,

    #[serde(rename = "lat")]
    pub lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    #[serde(rename = "temp")]
    pub temp: f64,

    #[serde(rename = "feels_like")]
    pub feels_like: f64,

    #[serde(rename = "temp_min")]
    pub temp_min: f64,

    #[serde(rename = "temp_max")]
    pub temp_max: f64,

    #[serde(rename = "pressure")]
    pub pressure: i64,

    #[serde(rename = "humidity")]
    pub humidity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub sys_type: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "sunrise")]
    pub sunrise: i64,

    #[serde(rename = "sunset")]
    pub sunset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "main")]
    pub main: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "icon")]
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    #[serde(rename = "speed")]
    pub speed: f64,

    #[serde(rename = "deg")]
    pub deg: i64,

    #[serde(rename = "gust")]
    pub gust: f64,
}
