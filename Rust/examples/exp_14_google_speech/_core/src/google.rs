use dotenv::dotenv;
use reqwest::Client;
use std::{env, error::Error};

/// Retrieves audio response from Google Text-to-Speech API.
pub async fn get_speech_from_api(text: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("GOOGLE_API_KEY").map_err(|_| "GOOGLE_API_KEY not found")?;
    let response = get_google_speech_response(text, "en-US", &api_key).await?;
    println!("{:#?}", response);
    let audio_content = response.bytes().await?;

    Ok(audio_content.to_vec())
}

/// Calls Google Text-to-Speech API to retrieve audio response.
async fn get_google_speech_response(
    text_to_speak: &str,
    language_code: &str,
    api_key: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let endpoint = "https://texttospeech.googleapis.com/v1/text:synthesize";
    let request_body = format!(
        r#"{{
            "input": {{"text": "{}"}},
            "voice": {{"languageCode": "{}"}},
            "audioConfig": {{
                "audioEncoding": "LINEAR16",
                "effectsProfileId": ["telephony-class-application"]
            }}
        }}"#,
        text_to_speak, language_code
    );
    let client = Client::new();

    client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to call Google Text-to-Speech API: {}", e).into())
}
