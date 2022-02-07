mod model;
use reqwest::header::AUTHORIZATION;
// use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A hard-coded JSON
    // let json = r#"
    //         {
    //           "main": {
    //             "temp": 30.94
    //           }
    //         }
    //     "#;

    // // // Deserialize the hardcoded JSON into a Weather struct
    // // let weather1: model::Weather = serde_json::from_str(json).unwrap();

    // println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    let client = reqwest::Client::new();
    let params = [("username", "sagar"), ("password", "9876")];
    // let mut map = HashMap::new();
    //map for username and password
    // map.insert("username", "username");
    // map.insert("password", "password");

    let response1 = client
        .post("http://localhost:3000/v1/auth")
        .form(&params)
        .send()
        .await?;
        
    let token_response = response1
        .json::<model::AuthT>()
        .await?;

    let response2 = client
        .get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, "Bearer ".to_owned()+&token_response.token)
        .send()
        .await?;

    let weather2 = response2
        .json::<model::Weather>()
        .await?;

    let response3 = client
        .get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, "Bearer ".to_owned()+&token_response.token)
        .send()
        .await?;
    
    let hello2 = response3
        .json::<model::Hello>()
        .await?;

    // println!("\nAuth :\n {:?}", token_response);
    println!("\nWeather :\n {:?}", weather2);
    println!("\nHello :\n {:?}", hello2);


    Ok(())
}
