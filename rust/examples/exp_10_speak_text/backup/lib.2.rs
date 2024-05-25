use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

pub enum PlaybackCommand {
    Play(Vec<u8>),
    // Add other commands as needed
}

pub struct MediaPlayer {
    sink: Arc<Mutex<Sink>>,
    command_sender: Sender<PlaybackCommand>,
}

impl MediaPlayer {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let (command_sender, command_receiver) = mpsc::channel::<PlaybackCommand>(32);
        let sink = Arc::new(Mutex::new(sink));

        // Spawn the background task for processing playback commands
        tokio::spawn(MediaPlayer::background_task(sink.clone(), command_receiver));

        Ok(MediaPlayer {
            sink,
            command_sender,
        })
    }

    async fn background_task(
        sink: Arc<Mutex<Sink>>,
        mut command_receiver: mpsc::Receiver<PlaybackCommand>,
    ) {
        while let Some(command) = command_receiver.recv().await {
            match command {
                PlaybackCommand::Play(audio_data) => {
                    let mut sink = sink.lock().await;
                    let source = Decoder::new(std::io::Cursor::new(audio_data))?;
                    sink.append(source);
                    sink.play();
                }
            }
        }
    }

    pub async fn play_audio(&self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.command_sender
            .send(PlaybackCommand::Play(audio_data))
            .await?;
        Ok(())
    }
}
