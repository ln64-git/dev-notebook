// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ws;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use surrealdb::{engine::local::RocksDb, Surreal};
use tokio::sync::{broadcast, Mutex};
use ws::WebSocketServer;

#[tokio::main]
async fn main() {
    // Initialize the database
    let db_path = "../database";
    let db = match Surreal::new::<RocksDb>(db_path).await {
        Ok(db) => db,
        Err(e) => {
            println!("Error opening database: {}", e);
            return;
        }
    };
    let _ = db.use_ns("test").use_db("test").await;
    let db_arc = Arc::new(Mutex::new(db));

    // Create broadcaster for WebSocket messages
    let (sender, _) = broadcast::channel::<String>(10);
    let sender_clone = sender.clone();
    let ws_server = Arc::new(Mutex::new(WebSocketServer {
        sender: sender.clone(),
        connected: false,
    }));

    // Spawn a Tokio task to start the WebSocket server
    tokio::spawn(async move {
        WebSocketServer::start(sender_clone).await;
    });

    // Tokio task to periodically broadcast chat entries
    tokio::spawn(async move {
        loop {
            if let Err(err) =
                broadcast_chat_entries(db_arc.clone(), sender.clone(), ws_server.clone()).await
            {
                println!("Error broadcasting chat entries: {}", err);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatEntry {
    timestamp: DateTime<Utc>,
    body: String,
}

async fn broadcast_chat_entries(
    db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>,
    sender: broadcast::Sender<String>,
    ws_server: Arc<Mutex<WebSocketServer>>, // Add a reference to the WebSocketServer
) -> Result<(), String> {
    let db = db_lock.lock().await;
    let chat_entries: Option<Vec<ChatEntry>> = match db.select("chat").await {
        Ok(entries) => Some(entries),
        Err(error) => return Err(format!("Database error: {:?}", error)),
    };
    let serialized_data = serde_json::to_string(&chat_entries).map_err(|e| e.to_string())?;

    let ws_server = ws_server.lock().await; // Lock the WebSocketServer
    if ws_server.connected {
        match sender.send(serialized_data) {
            Ok(_) => {
                println!("Message sent successfully.");
            }
            Err(err) => {
                println!("Failed to send message: {}", err);
            }
        }
    } else {
        println!("WebSocket connection not established, skipping message send.");
    }
    Ok(())
}
