// src/main.rs

// region: --- imports
mod _utils;
use actix_web::{web, App, HttpServer, Responder};
use core::{
    AppState, PlaybackCommand, PlaybackManager,
    _utils::{azure::speak_text, ollama::speak_ollama, playback},
};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

// Main Function
#[tokio::main]
async fn main() {
    println!("MAIN - Running main");

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let nexus = AppState {
        running: None,
        playback_send: playback::init_playback_channel().await,
    };
    println!("About to speak text...");

    let nexus_lock = Arc::new(Mutex::new(nexus));

    let playback_sender_clone = {
        let state = nexus_lock.lock().await;
        state.playback_send.clone()
    };

    let _ = speak_text("text", playback_sender_clone).await;

    println!("MAIN - function complete.");
}
