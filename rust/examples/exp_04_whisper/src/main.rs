//  src/main.rs
#![allow(dead_code)]

mod _utils;

use crate::_utils::record::record_audio;
use _utils::transcribe::speech_to_text;
use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let recording_output_path = Path::new("temp/audio.wav");
    // let resample_output_path = Path::new("temp/resampled_audio.wav");

    // Record audio
    record_audio(&recording_output_path).await?;

    // Process the resampled audio file with `speech_to_text`
    speech_to_text(&recording_output_path).await?;

    Ok(())
}
