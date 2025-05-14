use reqwest::Client;
use serde_json::Value;

type HandlerRequest = Result<(), Box<dyn std::error::Error>>;

struct Coordinates {
    lat: f64,
    lng: f64,
}
impl Coordinates {
    fn new(lat: f64, lng: f64) -> Self {
        Coordinates { lat, lng }
    }
}

struct Weather {
    weather_condition: String,
    temperature: f64,
    feels_like: f64,
    humidity: f64,
    precipitation_probability: f64,
}
impl Weather {
    fn new(weather_condition: String, temperature: f64, feels_like: f64, humidity: f64, precipitation_probability: f64) -> Self {
        Weather { weather_condition, temperature, feels_like, humidity, precipitation_probability }
    }
}


async fn get_coordinates(location: String, province: String, country: String, api_key: String) -> Result<Coordinates, Box<dyn std::error::Error>> {
    let url = format!(
        "https://maps.googleapis.com/maps/api/geocode/json?address={}+{}+{}&key={}",
        location.replace(" ", "+"), province.replace(" ", "+"), country.replace(" ", "+"), api_key
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let coordinates = Coordinates::new(
        response["results"][0]["geometry"]["location"]["lat"].as_f64().unwrap(),
        response["results"][0]["geometry"]["location"]["lng"].as_f64().unwrap(),
    );

    Ok(coordinates)
}

async fn get_weather(location_coord: Coordinates, api_key: String) -> Result<Weather, Box<dyn std::error::Error>> {
    let url = format!(
        "https://weather.googleapis.com/v1/currentConditions:lookup?key={}&location.latitude={}&location.longitude={}",
        api_key, location_coord.lat, location_coord.lng
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Value>()
        .await?;
    let weather = Weather::new(
        response["weatherCondition"]["description"]["text"].as_str().unwrap().to_string(),
        response["temperature"]["degrees"].as_f64().unwrap(),
        response["feelsLikeTemperature"]["degrees"].as_f64().unwrap(),
        response["relativeHumidity"].as_f64().unwrap(),
        response["precipitation"]["probability"]["percent"].as_f64().unwrap(),
    );
    Ok(weather)
} 


#[tokio::main]
async fn main() -> HandlerRequest {
    let api_key: String = "AIzaSyBoLzYE1EY5v1YH-bzJpF8PjIpSDjnj1ek".to_string();
    let location: String = "San José".to_string();
    let province: String = "San José".to_string();
    let country: String = "Costa Rica".to_string();

    let coordinates_data = get_coordinates(location, province, country, api_key.clone()).await?;

    let weather_data = get_weather(coordinates_data, api_key.clone()).await?;

    Ok(())
}
