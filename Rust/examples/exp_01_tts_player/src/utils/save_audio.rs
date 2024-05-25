use crate::File;
use std::io::Write;

pub(crate) fn save_audio(
    audio_content: &[u8],
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(audio_content)?;

    println!("Audio content saved to {}", file_path);

    Ok(())
}
