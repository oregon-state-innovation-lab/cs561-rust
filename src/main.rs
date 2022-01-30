use reqwest::header::AUTHORIZATION;

mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // A hard-coded JSON
    // let json = r#"
    //         {
    //           "main": {
    //             "temp": 30.94
    //           }
    //         }
    //     "#;
    //
    // // Deserialize the hardcoded JSON into a Weather struct
    // let weather1: model::Weather = serde_json::from_str(json).unwrap();
    //
    // println!("\nWeather from a JSON we hard-coded locally:\n{:?}", weather1);

    //
    // Now that we know we can deserialize a hard-coded JSON into a struct model,
    // let's see if we can fetch the weather from the backend.
    //

    let client = reqwest::Client::new();

    // Get the token by calling the login API
    let params = [("username", "test-user"), ("password", "test-password")];
    let token_response = client.post("http://localhost:3000/v1/auth")
        .form(&params)
        .send()
        .await?;

    let token  = token_response
        .json::<model::Token>()
        .await?;

    println!("\nReceived Token:\n {:?}", token);

    let response = client
        .get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, "Bearer ".to_owned() + &token.token)
        .send()
        .await?;

    println!("\nResponse from Weather Service Status:\n {:?}", response.status());

    let weather2 = response
        .json::<model::Weather>()
        .await?;

    println!("\nTemperature from Weather Service:\n {:?}", weather2);

    Ok(())
}
