// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ws;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc, thread, time::Duration};
use surrealdb::{engine::local::RocksDb, Surreal};
use tauri::Manager;
use tokio::sync::{broadcast, Mutex};
use ws::start_websocket_server;

#[derive(Debug)]
pub struct AppState {
    pub db: Surreal<surrealdb::engine::local::Db>,
}

#[tokio::main]
async fn main() {
    let db_path = "../database";
    let db = match Surreal::new::<RocksDb>(db_path).await {
        Ok(db) => db,
        Err(e) => {
            println!("Error opening database: {}", e);
            return;
        }
    };
    let _ = db.use_ns("test").use_db("test").await;

    let nexus = Arc::new(Mutex::new(AppState { db: db.clone() }));
    let db_arc = Arc::new(Mutex::new(db));

    let (sender, _) = broadcast::channel::<String>(10); // Create broadcaster
    let sender_clone = sender.clone();

    thread::spawn(move || {
        ws::start_websocket_server(sender_clone);
    });
    // tauri::async_runtime::spawn(start_websocket_server(sender_clone));
    // thread::spawn(start_websocket_server(sender_clone));

    tokio::spawn(async move {
        loop {
            let sender = sender.clone();
            if let Err(err) = broadcast_chat_entries(db_arc.clone(), sender.clone()).await {
                println!("Error broadcasting chat entries: {}", err);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_input_from_frontend,])
        .manage(nexus.clone())
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
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
) -> Result<(), String> {
    let db = db_lock.lock().await;
    let chat_entries: Option<Vec<ChatEntry>> = match db.select("chat").await {
        Ok(entries) => Some(entries),
        Err(error) => return Err(format!("Database error: {:?}", error)),
    };
    let serialized_data = serde_json::to_string(&chat_entries).map_err(|e| e.to_string())?;
    match sender.send(serialized_data) {
        Ok(_) => {
            println!("Message sent successfully.");
        }
        Err(err) => {
            println!("Failed to send message: {}", err);
        }
    }
    Ok(())
}

#[tauri::command]
async fn process_input_from_frontend(text: String, app: tauri::AppHandle) -> Result<(), String> {
    let db = app.state::<Arc<Mutex<AppState>>>().lock().await.db.clone();
    let _ = add_chat_entry_to_db(Arc::new(Mutex::new(db)), text).await;
    Ok(())
}

async fn add_chat_entry_to_db(
    db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>,
    content: String,
) -> Result<(), Box<dyn Error>> {
    let db = db_lock.lock().await;
    let content = ChatEntry {
        timestamp: Utc::now(),
        body: content,
    };
    let _: Option<Vec<ChatEntry>> = match db.create("chat").content(content).await {
        Ok(records) => {
            records.clone().into_iter().next();
            Some(records)
        }
        Err(e) => {
            println!("PROCESS_INPUT - Error: {:?}", e);
            None
        }
    };
    Ok(())
}
