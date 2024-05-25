use exp_06_thread_control::AppState;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let nexus = Arc::new(Mutex::new(AppState::new()));
    let (tx, mut rx) = mpsc::channel::<String>(10); // Channel for command messages

    // Spawn an async task to read commands
    tokio::spawn(async move {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if tx.send(line).await.is_err() {
                eprintln!("Failed to send command.");
                break;
            }
        }
    });

    println!("Enter command:");
    while let Some(command) = rx.recv().await {
        match process_commands(&command, Arc::clone(&nexus)).await {
            Err(e) => println!("Error processing command: {}", e),
            Ok(_) => (),
        }
    }
}

async fn process_commands(
    command: &str,
    nexus: Arc<Mutex<AppState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("processing command");
    match command.trim() {
        "start" => launch_main_thread(nexus).await,
        "stop" => stop_main_thread(nexus).await,
        _ => Ok(()),
    }
}

async fn launch_main_thread(
    nexus: Arc<Mutex<AppState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = nexus.lock().await;
    if state.running.is_none() {
        println!("Initializing main thread");

        let (tx, mut rx) = mpsc::channel(1);
        state.running = Some(tx);

        tokio::spawn(async move {
            let mut count = 0;
            loop {
                tokio::select! {
                    _ = sleep(Duration::from_secs(1)) => {
                        println!("{}", count);
                        count += 1;
                    },
                    _w = rx.recv() => {
                        println!("Stopping count");
                        break;
                    }
                }
            }
        });

        Ok(())
    } else {
        println!("Counter is already running.");
        Ok(())
    }
}

async fn stop_main_thread(
    nexus: Arc<Mutex<AppState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = nexus.lock().await;
    if let Some(tx) = state.running.take() {
        println!("Attempting to stop main thread...");
        let _ = tx.send(()).await;
        println!("Main thread stopped.");
    } else {
        println!("Main thread not running.");
    }
    Ok(())
}
