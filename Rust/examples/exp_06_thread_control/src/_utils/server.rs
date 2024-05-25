// region: --- imports
use crate::{
    AppState,
    _test::{
        counter::{start_counter, stop_counter},
        test::test_endpoint,
    },
};
use actix_web::{web, App, HttpServer, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;
// endregion: --- imports

fn register_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/start", web::get().to(start_counter))
        .route("/stop", web::get().to(stop_counter))
        .route("/test", web::get().to(test_endpoint));
}

pub async fn start_server(nexus: Arc<Mutex<AppState>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(nexus.clone()))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
