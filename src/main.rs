use axum::{routing::post, Router};
use std::net::SocketAddr;

#[allow(non_snake_case)]
mod api {
    pub mod calcController;
}

#[allow(non_snake_case)]
mod models {
    pub mod calcRequest;
    pub mod calcResponse;
}

#[allow(non_snake_case)]
mod services {
    pub mod cloudCalc;
    pub mod compareService;
    pub mod localCalc;
    pub mod sensitivityService;
}
// ---------------------------------------------------------

#[tokio::main]
async fn main() {
    // init log
    tracing_subscriber::fmt::init();

    // route processing
    let app = Router::new()
        // POST processing
        .route("/api/calc", post(api::calcController::process_calculation));

    // address for docker
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // run server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}