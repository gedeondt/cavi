use axum::{
    extract::{Path, State, Query},
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use kvstore::{KvStore, engine::mem::MemStore};

use crate::shard::router::ShardRouter;

#[derive(Clone)]
pub struct AppState {
    store: Arc<Mutex<MemStore>>,
    router: Arc<ShardRouter>,
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

pub fn build_app(router: Arc<ShardRouter>) -> Router {
    let store = MemStore::new();
    let state = AppState {
        store: Arc::new(Mutex::new(store)),
        router,
    };

    Router::new()
        .route("/kv/:key", get(get_key).put(set_key).delete(delete_key))
        .route("/search", get(search_by_prefix))
        .with_state(state)
}

// Handlers
async fn get_key(Path(key): Path<String>, State(state): State<AppState>) -> Json<Option<String>> {
    let store = state.store.lock().unwrap();
    Json(store.get(&key).unwrap())
}

async fn set_key(Path(key): Path<String>, State(state): State<AppState>, Json(payload): Json<SetRequest>) {
    let mut store = state.store.lock().unwrap();
    store.set(key, payload.value).unwrap();
}

async fn delete_key(Path(key): Path<String>, State(state): State<AppState>) {
    let mut store = state.store.lock().unwrap();
    store.delete(&key).unwrap();
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
