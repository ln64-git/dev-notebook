// lib.rs

// region: --- imports
pub mod _utils;
use _utils::azure;
use _utils::ollama;
use _utils::playback;
use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Cursor;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
// endregion: --- imports

#[derive(Debug)]
pub struct AppState {}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {}
    }
}
