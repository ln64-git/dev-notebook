// lib.rs

use tokio::sync::mpsc;

pub struct AppState {
    pub running: Option<mpsc::Sender<()>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { running: None }
    }
}
