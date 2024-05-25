// region: --- imports
mod _utils;
use chrono::prelude::*;
use core::AppState;
use core::_utils::azure::get_azure_audio_response;
use dotenv::dotenv;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::env;
use std::error::Error;
use std::io::{self, Cursor};
use tokio::sync::oneshot;
// endregion: --- imports

async fn speak_text(text: &str) -> Result<(), Box<dyn Error>> {
    let audio_data = get_azure_audio_response("Hello?").await?;
    if let Err(err) = play_audio(audio_data).await {
        eprintln!("Error playing audio: {:?}", err);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let start_time = Utc::now();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    speak_text("Hello?").await;

    let end_time = Utc::now(); // Record end time
    let duration = end_time.signed_duration_since(start_time); // Calculate duration
    let seconds = duration.num_seconds(); // Extract seconds from duration
    println!("Execution time: {} seconds", seconds); // Print execution time
}

async fn play_audio(audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    // Create a shared output stream for audio playback
    let (stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Create a decoder from the audio data
    let source = Decoder::new(Cursor::new(audio_data))?;

    // Append the audio source to the sink
    sink.append(source);

    // Spawn a separate task to handle user input
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        let _ = tx.send(handle_user_input());
    });

    // Block until the audio playback is finished or interrupted by user input
    sink.sleep_until_end();

    // Wait for user input task to complete
    let _ = rx.await;

    Ok(())
}

fn handle_user_input() {
    println!("Audio playback started. Type 'pause', 'resume', or 'stop' to control playback.");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "pause" => {
                println!("Audio paused.");
                // Implement pause functionality
            }
            "resume" => {
                println!("Audio resumed.");
                // Implement resume functionality
            }
            "stop" => {
                println!("Audio stopped.");
                // Implement stop functionality
                break;
            }
            _ => println!("Invalid input. Type 'pause', 'resume', or 'stop'."),
        }
    }
}
