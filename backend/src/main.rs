use axum::{
    extract::Json,
    http::Method,
    response::Html,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct IncomingRequest {
    timestamp: String,
    body: serde_json::Value,
}

type SharedState = Arc<Mutex<Vec<IncomingRequest>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", post(handle_post))
        .route("/requests", get(get_requests))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend running at http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_post(
    state: axum::extract::State<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> &'static str {
    let mut requests = state.lock().unwrap();
    requests.push(IncomingRequest {
        timestamp: Utc::now().to_rfc3339(),
        body: payload,
    });
    "ok"
}

async fn get_requests(
    state: axum::extract::State<SharedState>,
) -> Json<Vec<IncomingRequest>> {
    let requests = state.lock().unwrap().clone();
    Json(requests.into_iter().rev().collect())
}

