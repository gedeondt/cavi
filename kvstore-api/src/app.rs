use axum::{
    extract::{Path, State, Query},
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use kvstore::{KvStore, engine::mem::MemStore};
use axum::http::StatusCode; 
use axum::response::IntoResponse;
use crate::shard::router::ShardRouter;
use crate::remote::RemoteNodeClient;

#[derive(Clone)]
pub struct AppState {
    store: Arc<Mutex<MemStore>>,
    router: Arc<ShardRouter>,
    client: Arc<dyn RemoteNodeClient>,
}

#[derive(Deserialize)]
struct SetRequest {
    value: String,
}

#[derive(Serialize)]
struct KeyValue {
    key: String,
    value: String,
}

pub fn build_app(router: Arc<ShardRouter>, client: Arc<dyn RemoteNodeClient>) -> Router {
    let state = AppState {
        store: Arc::new(Mutex::new(MemStore::new())),
        router,
        client,
    };

    Router::new()
        .route("/kv/:key", get(get_key).put(set_key).delete(delete_key))
        .route("/search", get(search_by_prefix))
        .with_state(state)
}

// Handlers
async fn get_key(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("[GET] key = '{}'", key);

    if !state.router.is_local(&key) {
        let remote = state.router.address_for_key(&key);
        return match state.client.forward_get(&key, &remote).await {
            Ok(Some(val)) => Json(val).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(e) => {
                eprintln!("forward_get error: {}", e);
                StatusCode::BAD_GATEWAY.into_response()
            }
        };
    }

    let store = state.store.lock().unwrap();
    match store.get(&key) {
        Ok(Some(val)) => Json(val).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn set_key(
    Path(key): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<SetRequest>,
) -> impl IntoResponse {
    println!("[SET] key = '{}', value = '{}'", key, payload.value);

    if !state.router.is_local(&key) {
        let remote = state.router.address_for_key(&key);
        return match state.client.forward_set(&key, &payload.value, &remote).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(e) => {
                eprintln!("forward_set error: {}", e);
                StatusCode::BAD_GATEWAY
            }
        };
    }

    let mut store = state.store.lock().unwrap();
    match store.set(key, payload.value) {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn delete_key(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("[DELETE] key = '{}'", key);

    if !state.router.is_local(&key) {
        let remote = state.router.address_for_key(&key);
        return match state.client.forward_delete(&key, &remote).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(e) => {
                eprintln!("forward_delete error: {}", e);
                StatusCode::BAD_GATEWAY
            }
        };
    }

    let mut store = state.store.lock().unwrap();
    match store.delete(&key) {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize)]
struct SearchParams {
    prefix: String,
}

async fn search_by_prefix(Query(params): Query<SearchParams>, State(state): State<AppState>) -> Json<Vec<KeyValue>> {
    let store = state.store.lock().unwrap();
    let result = store
        .search_by_prefix(&params.prefix)
        .unwrap()
        .into_iter()
        .map(|(k, v)| KeyValue { key: k, value: v })
        .collect();
    Json(result)
}
