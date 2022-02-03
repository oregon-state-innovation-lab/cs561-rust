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
    
    //the fake user in the user database to revieve auth token.
    let mut map = HashMap::new();
    map.insert("username", "bharath");
    map.insert("password", "pass");


    //auth endpoint, this is the post request and will return an auth token to
    //the user, will be basis for the hello and weather endpoints. 
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000/v1/auth")
        .json(&map)
        .send()
        .await?;
    let auth_token = res
        .json::<model::Token>()
        .await?;
    println!("\nToken from backend service:\n {:?}\n", auth_token);
    let header_value = format!("Bearer {}", auth_token.accessToken);


    //The hello endpoint, need to clone the auth so the weather request can
    //pass it to the server aswell
    let client2 = reqwest::Client::new();

    let hello_res = client2.get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, header_value.clone())
        .send()
        .await?;
    
    let hello_message = hello_res
        .json::<model::Hello>()
        .await?;

    println!("Hello from Node Backend:\n {:?}\n", hello_message);
    

    //The weather endpoint, uses the cloned auth from the previous hello
    //request to get the current weather from the node server
    let client3 = reqwest::Client::new();

    let weather_res = client3.get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, header_value)
        .send()
        .await?;
    
    let weather_message = weather_res
        .json::<model::Weather>()
        .await?;

    println!("\nWeather from Node Backend:\n {:?}\n", weather_message);


    Ok(())
}
