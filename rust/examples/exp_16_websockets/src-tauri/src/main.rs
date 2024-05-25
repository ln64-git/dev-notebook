pub mod ws;
use std::thread;
use ws::start_websocket_server;

#[tokio::main]
async fn main() {
    // Start the WebSocket server in a separate thread
    thread::spawn(start_websocket_server);

    // Start the Tauri application
    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
