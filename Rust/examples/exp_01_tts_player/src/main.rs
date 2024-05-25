use dotenv::dotenv;
use reqwest::Error as ReqwestError;
use std::{env, fs::File};
use utils::{azure_tts::get_azure_response, save_audio::save_audio};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found");
    let region = "eastus";
    let text_to_speak = "Hello, world!";
    let voice_name = "en-US-AriaRUS";

    let response = get_azure_response(&api_key, region, text_to_speak, voice_name).await?;
    let audio_content = response.bytes().await?.to_vec();

    save_audio(&audio_content, "output.wav")?;

    Ok(())
}
