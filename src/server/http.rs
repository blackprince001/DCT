use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::{extract::State, routing::get, Router};

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

type SharedState = Arc<RwLock<crate::network::types::NetworkAnalytics>>;

#[derive(Clone)]
pub struct AppState {
    network_analytics: SharedState,
}

async fn ws_metrics_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| handle_ws_connection(socket, state))
}

async fn handle_ws_connection(mut socket: WebSocket, state: AppState) {
    let mut interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let metrics = state.network_analytics.read().await.get_metrics();
                if let Ok(json) = serde_json::to_string(&metrics) {
                    if socket.send(axum::extract::ws::Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }
}

pub fn create_router(network_state: SharedState) -> Router {
    let state = AppState {
        network_analytics: network_state,
    };

    Router::new()
        .route("/ws", get(ws_metrics_handler))
        .with_state(state)
}
