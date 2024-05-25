// region: --- imports
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;
use tokio::time::Duration;
// endregion: --- imports

pub async fn start_counter(
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let nexus = data.into_inner();
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

        HttpResponse::Ok().body("Counter started")
    } else {
        HttpResponse::Ok().body("Counter is already running")
    }
}

pub async fn stop_counter(
    data: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    let nexus = data.into_inner();
    let mut state = nexus.lock().await;
    if let Some(tx) = state.running.take() {
        println!("Attempting to stop main thread...");
        let _ = tx.send(()).await;
        println!("Main thread stopped.");
        HttpResponse::Ok().body("Counter stopped")
    } else {
        HttpResponse::Ok().body("Counter was not running")
    }
}
