#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_get_audio_response() {
        // Define test data
        let api_key = "your-api-key";
        let region = "eastus";
        let text_to_speak = "Hello, world!";
        let voice_name = "en-US-AriaRUS";

        // Call the function and await the result
        let response_result = get_audio_response(api_key, region, text_to_speak, voice_name).await;

        // Assert that the result is Ok
        assert!(response_result.is_ok());

        // If needed, you can further validate the response contents
        let response = response_result.unwrap();
        // For example, you can check the status code
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        // You can also check response headers, body, etc.
    }
}
