// region: --- imports
use actix_web::{web, App, HttpServer, Responder};
use exp_06_thread_control::PlaybackCommand;
use exp_06_thread_control::PlaybackManager;
use exp_06_thread_control::_utils::ollama::speak_ollama;
use exp_06_thread_control::_utils::server::start_server;
use exp_06_thread_control::{
    AppState,
    _test::{
        counter::{start_counter, stop_counter},
        test::test_endpoint,
    },
    _utils::playback,
    _utils::server,
};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let nexus = AppState {
        running: None,
        playback_send: playback::init_playback_channel().await,
    };

    let nexus_lock = Arc::new(Mutex::new(nexus));

    let playback_sender_clone = {
        let state = nexus_lock.lock().await;
        state.playback_send.clone()
    };
    println!("Doesn't work outside of REST");
    // match speak_ollama("What's my name".to_owned(), playback_sender_clone).await
    // {
    //     Ok(_) => println!("Successfully spoke llama"),
    //     Err(e) => println!("Error speaking llama: {}", e),
    // }
}
