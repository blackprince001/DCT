use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::{extract::State, response::Json, routing::get, Router};

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

use crate::network::db::{MetricsStorage, SqliteStorage};
use crate::network::types::{HourlySample, NetworkAnalytics, NetworkMetrics};

type SharedState = Arc<RwLock<NetworkAnalytics>>;

#[derive(Clone)]
pub struct AppState {
    network_analytics: SharedState,
    db: Arc<SqliteStorage>,
}

async fn get_metrics(State(state): State<AppState>) -> Json<NetworkMetrics> {
    let analytics = state.network_analytics.read().await;
    Json(analytics.get_metrics())
}

async fn get_hourly_metrics(State(state): State<AppState>) -> Json<Vec<HourlySample>> {
    match state.db.get_hourly_samples().await {
        Ok(samples) => Json(samples),
        Err(e) => {
            eprintln!("Error fetching hourly samples: {}", e);
            Json(Vec::new())
        }
    }
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

pub fn create_router(network_state: SharedState, database: SqliteStorage) -> Router {
    let state = AppState {
        network_analytics: network_state,
        db: Arc::new(database),
    };

    Router::new()
        .route("/metrics", get(get_metrics))
        .route("/metrics/hourly", get(get_hourly_metrics))
        .route("/ws", get(ws_metrics_handler))
        .with_state(state)
}
