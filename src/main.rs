mod model;
use std::collections::HashMap;
use reqwest::header::AUTHORIZATION;

#[tokio::main]
#small change to rust file
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    let json = r#"
            {
              "main": {
                "temp": 30.94
              }
            }
        "#;

    // Deserialize the hardcoded JSON into a Weather struct
    let weather1: model::Weather = serde_json::from_str(json).unwrap();

    println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    // Authorize user
    let mut map = HashMap::new();
    map.insert("username", "John");
    map.insert("password", "Doe");

    let client1 = reqwest::Client::new();
    let response1 = client1
        .post("http://localhost:3000/v1/auth")
        .json(&map)
        .send()
        .await?;
    let acc_token = response1
        .json::<model::Auth>()
        .await?;
    println!("\nAccess token:\n {:?}", acc_token.accesstoken);
    let _header_value = format!("Bearer {}", acc_token.accesstoken);

    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //
    // .get("https://api.openweathermap.org/data/2.5/weather?q=corvallis&appid=[INSERT_API_KEY_HERE}")

    let client2 = reqwest::Client::new();

    let response2 = client2
        .get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, _header_value.clone())
        .send()
        .await?;

    let weather2 = response2
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from mock server:\n {:?}", weather2.main.temp);

    // Get greeting
    
    let client3 = reqwest::Client::new();

    let response3 = client3
        .get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, _header_value.clone())
        .send()
        .await?;
      
    let greeting = response3
        .json::<model::Hello>()
        .await?;

    println!("\nGreeting: \n {:?}", greeting.greeting);
    Ok(())
}
