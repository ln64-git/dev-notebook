// region: --- Region Title
pub mod google;
pub mod playback;
use crate::google::get_speech_from_api;
use playback::PlaybackCommand;
use std::error::Error;
use tokio::sync::mpsc;
// endregion: --- Region Title

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
        }
    }
}

pub async fn speak_text(
    text: &str,
    playback_send: &mpsc::Sender<PlaybackCommand>,
) -> Result<(), Box<dyn Error>> {
    let audio_data = get_speech_from_api(text).await?;
    let _ = playback_send.send(PlaybackCommand::Play(audio_data)).await;
    Ok(())
}
