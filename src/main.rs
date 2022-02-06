mod model;

use reqwest::header::AUTHORIZATION;
use crate::model::SiteUser;

// Author: Matthew Pacey

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // using aws ec2 instance

    let client = reqwest::Client::new();
    let u = SiteUser{
        username: "joe".into(),
        password: "my_password2".into(),
    };

    let response = client
        .post("http://55.55.131.241:3000/v1/auth/")
        .json(&u)
        .send()
        .await?;

    //println!("Response {:?}", response);
    let auth_json = response
        .json::<model::Token>()
        .await?;

    //println!("auth token {:?}", auth_json.main.token);
    let token = "Bearer ".to_owned() + &auth_json.main.token;

    // check weather response
    let weather_response = client
        .get("http://55.55.131.241:3000/v1/weather/")
        .header(AUTHORIZATION, token)
        .send()
        .await?;

    let weather = weather_response
        .json::<model::Weather>()
        .await?;

    println!("Temperature from weather service {:?}", weather);


    // check hello world response
    let hello_token = "Bearer ".to_owned() + &auth_json.main.token;
    let hello_response = client
        .get("http://55.55.131.241:3000/v1/hello/")
        .header(AUTHORIZATION, hello_token)
        .send()
        .await?;

    let hello = hello_response
        .json::<model::Hello>()
        .await?;

    println!("Greeting from Hello: {:?}", hello.main.greeting);

    Ok(())
}
