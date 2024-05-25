// region: --- imports
use crate::{
    AppState,
    _utils::{azure::speak_text, ollama::speak_ollama},
};
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

pub async fn test_endpoint(
    nexus: web::Data<Arc<Mutex<AppState>>>,
) -> impl Responder {
    // let _ = start_counter(nexus.clone()).await;
    // let _ = speak_text("Hello World!", state.playback_send.clone()).await;

    let state = nexus.lock().await;

    let _ = speak_ollama(
        "What does the name Luke represent?".to_owned(),
        state.playback_send.clone(),
    )
    .await;

    HttpResponse::Ok().body("Test Complete.")
}
