use serde::{Deserialize, Serialize};
use std::error::Error;
use reqwest::blocking::{Client, Response};
use serde_json::Value;

const API_URL: &str = "https://openapi.youdao.com/api";
const APP_KEY: &str = "48de8104f7631356";
const APP_SECRET: &str = "0dalavuogFk8Fl6ZyHa6Mj13mmVwLAqA";

#[derive(Serialize)]
struct TranslationRequest<'a> {
    q: &'a str,
    from: &'a str,
    to: &'a str,
    appKey: &'a str,
    salt: i32,
    sign: String,
}

#[derive(Deserialize)]
struct TranslationResponse {
    translation: Vec<String>,
}

fn translate_english_to_chinese(text: &str) -> Result<String, Box<dyn Error>> {
    let from = "en";
    let to = "zh-CHS";
    let salt = rand::random::<i32>();
    let sign = format!(
        "{}{}{}{}",
        APP_KEY,
        text,
        salt,
        APP_SECRET
    );
    let sign = md5::compute(sign);
    let sign = format!("{:x}", sign);

    let request = TranslationRequest {
        q: text,
        from,
        to,
        appKey: APP_KEY,
        salt,
        sign,
    };

    let client = Client::new();
    let response: Response = client.post(API_URL)
        .form(&request)
        .send()?;

    let response_json: Value = response.json()?;
    let translation = response_json["translation"][0]
        .as_str()
        .ok_or("Missing translated text")?;

    Ok(translation.to_owned())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter some English text to translate to Chinese:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let translation = translate_english_to_chinese(&input)?;
    println!("Chinese translation: {}", translation);
    Ok(())
}
