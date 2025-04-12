pub mod auth_handler;
pub mod post_handler;

use dotenvy::dotenv;
use reqwest::header::CONTENT_TYPE;
use reqwest::Error;
use rocket::serde::Deserialize;
use serde_json::json;
use std::env;

pub async fn analyze_for_ai() -> Result<u8, Error> {
    dotenv().expect("Unable to load dotenv file.");
    let gemini_api_key =
        env::var("GEMINI_API_KEY").expect("Unable to load the JWT_KEY env variable.");
    let body = json!({
    "contents": [{
      "parts":[{"text": "Explain how AI works"}]
      }]
     });

    let request_url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", gemini_api_key);
    let client = reqwest::Client::new();
    let response = client
        .post(request_url)
        .header(CONTENT_TYPE, "application/json")
        .body(body.to_string())
        .send()
        .await?;

    // will need to deserialize a struct that resembles the gemini http response structure
    // use cURL in the terminal to see the http response structure
    //println!("{:?}", response.json());
    Ok(0)
}
