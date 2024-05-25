use multi_threaded_media_player::speak_ollama;
use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use multi_threaded_media_player::AppState;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[actix_web::main] // Marks the entry point of the application
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Defines a route
    })
    .bind("127.0.0.1:8080")? // Specifies the address and port to serve on
    .run()
    .await
}

// Ollama content to speech
pub async fn speak_ollama_endpoint(
    body: web::Json<String>,
    data: web::Data<Mutex<AppState>>,
) -> impl Responder {
    let preface = "In three sentences explain...";
    let final_prompt = format!("{} {}", preface, *body);

    let control_tx = {
        let lock = data.lock().unwrap();
        lock.control_tx.clone()
    };

    match speak_ollama(final_prompt, control_tx).await {
        Ok(_) => HttpResponse::Ok().body("Ollama content spoken."),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
