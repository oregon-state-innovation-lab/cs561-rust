use reqwest::header::AUTHORIZATION;

mod model;
mod weather_model;
mod greetings_model;
mod authentication_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    call_weather_service().await?;
    call_greetings_service().await?;

    Ok(())
}

async fn call_authentication_service() -> Result<String, Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    // Calling the Greeting API
    // Get the token by calling the login API
    let params = [("username", "test-user"), ("password", "test-password")];
    let token_response = client.post("http://localhost:3000/v1/auth")
        .form(&params)
        .send()
        .await?;
    println!("\nResponse from Authentication Service Status:\n {:?}", token_response.status());

    let token_model  = token_response
        .json::<authentication_model::AuthenticationModel>()
        .await?;

    println!("\nReceived Token:\n {:?}", token_model);

    Ok(token_model.access_token)
}

async fn call_greetings_service() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n******** Greetings Service ********");

    let client = reqwest::Client::new();

    // Calling the Greeting API
    // Get the token by calling the login API
    let greetings_response = client
        .get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, "Bearer ".to_owned() + call_authentication_service().await?.as_str())
        .send()
        .await?;

    println!("\nResponse from Greetings Service:\n {:?}", greetings_response.status());

    let greetings_model = greetings_response
        .json::<greetings_model::GreetingsModel>()
        .await?;

    println!("\nResponse from Greetings Service:\n {:?}", greetings_model);

    Ok(())
}


async fn call_weather_service() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n******** Weather Service ********");

    let client = reqwest::Client::new();

    let weather_response = client
        .get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, "Bearer ".to_owned() + call_authentication_service().await?.as_str())
        .send()
        .await?;

    println!("\nResponse from Weather Service Status:\n {:?}", weather_response.status());

    let weather_model = weather_response
        .json::<weather_model::WeatherModel>()
        .await?;

    println!("\nResponse from Weather Service:\n {:?}", weather_model);

    Ok(())
}


