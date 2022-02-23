use std::collections::HashMap;
use reqwest::header::AUTHORIZATION;
mod model;

#[tokio::main]
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
    
    // This will POST a body of `{"lang":"rust","body":"json"}`
    //Reference : https://docs.rs/reqwest/latest/reqwest/
    let mut map = HashMap::new();
    map.insert("username", "atharva");
    map.insert("password", "admin");

    let auth_client = reqwest::Client::new();
    let auth_response = auth_client
      .post("http://localhost:3001/v1/auth") 
      .json(&map)
      .send()
      .await?;

    println!("\nMessage from Auth URL:\n {:?}", auth_response);
    let auth_token = auth_response
      .json::<model::AuthToken>()
      .await?;
    
    let token = auth_token.token;

    let client1 = reqwest::Client::new();
    let response1 = client1
        .get("http://localhost:3001/v1/Weather")
        .header(AUTHORIZATION, &token)
        .send()
        .await?;
    let weather2 = response1
        .json::<model::Weather>()
        .await?;
    println!("\nWeather from openweathermap.org:\n {:?}", weather2);
 
    let client2 = reqwest::Client::new();
    let response2 = client2
        .get("http://localhost:3001/v1/hello")
        .header(AUTHORIZATION,&token)
        .send()
        .await?;

    let hello = response2
        .json::<model::Hello>()
        .await?;
    
    println!("\nMessage from Greeting URL:\n {:?}", hello);
    Ok(())
}