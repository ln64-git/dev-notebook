// lib.rs

use rodio::{Decoder, OutputStream, Sink};
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;

pub enum PlaybackCommand {
    Play(Vec<u8>),
    // Add other commands as needed
}

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MediaPlayer {
    pub sink: Option<Sink>,
    pub command_queue: Arc<Mutex<VecDeque<PlaybackCommand>>>, // Define a queue for playback commands wrapped in a Mutex
    pub is_idle: AtomicBool, // Define a flag to track the playback status
}

impl MediaPlayer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Initialize the command queue and set the is_idle flag to true
        let command_queue = Arc::new(Mutex::new(VecDeque::new()));
        let is_idle = AtomicBool::new(true);

        Ok(MediaPlayer {
            sink: None,
            command_queue,
            is_idle,
        })
    }

    pub async fn play_audio(&self, audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        // Create a new sink for each audio playback to avoid blocking calls
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        // Append audio data to the sink
        let source = Decoder::new(Cursor::new(audio_data))?;
        sink.append(source);

        self.sink = sink;

        // Sleep for the duration of the audio playback asynchronously
        sink.sleep_until_end();

        Ok(())
    }

    async fn process_command(&self) {
        // Acquire a lock on the command queue
        let mut queue = self.command_queue.lock().await;

        // Process the command by dequeuing from the command queue and playing the audio
        while let Some(command) = queue.pop_front() {
            match command {
                PlaybackCommand::Play(audio_data) => {
                    if let Err(err) = self.play_audio(audio_data).await {
                        eprintln!("Error playing audio: {:?}", err);
                    }
                } // Handle other commands as needed
            }
        }
    }
}
