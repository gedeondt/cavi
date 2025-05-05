use axum::{
    extract::{Path, State, Query},
    routing::{get},
    Json, Router,
};

use std::sync::{Arc, Mutex};
use kvstore::{KvStore, engine::mem::MemStore};
use axum::http::StatusCode; 
use axum::response::IntoResponse;
use crate::shard::router::ShardRouter;
use crate::remote::RemoteNodeClient;
use futures::future::join_all;
use crate::types::{SetRequest, KeyValue, SearchParams};
use axum::http::HeaderMap;
use axum::extract::Request;
use axum::body::Body;

#[derive(Clone)]
pub struct AppState {
    store: Arc<Mutex<MemStore>>,
    router: Arc<ShardRouter>,
    client: Arc<dyn RemoteNodeClient>,
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

pub async fn search_by_prefix(
    Query(params): Query<SearchParams>,
    State(state): State<AppState>,
    req: Request<Body>, // ✅ para acceder a las cabeceras
) -> impl IntoResponse {
    let prefix = params.prefix.clone();
    let my_id = state.router.my_id;

    println!("🔍 Node {} received /search?prefix={}", my_id, prefix);

    // ✅ Leer cabecera para evitar reenvíos circulares
    let headers: &HeaderMap = req.headers();
    let forwarded = headers.get("x-forwarded-search").is_some();

    if forwarded {
        println!("↪️ Node {}: This is a forwarded request, no further forwarding.", my_id);
    } else {
        println!("🚀 Node {}: This is an original request, will forward to other nodes.", my_id);
    }

    // 🧠 Búsqueda local
    let local_result = {
        let store = state.store.lock().unwrap();
        let matches: Vec<_> = store
            .search_by_prefix(&prefix)
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| KeyValue { key: k, value: v })
            .collect();

        println!("📦 Node {}: Found {} local matches", my_id, matches.len());
        matches
    };

    // 🚫 Si reenviada, devolvemos solo local
    if forwarded {
        return Json(local_result);
    }

    // ✅ Reenviar a los demás nodos
    let remote_tasks = state
        .router
        .shards
        .iter()
        .filter(|shard| shard.id != my_id)
        .map(|shard| {
            let client = state.client.clone();
            let prefix = prefix.clone();
            let addr = shard.addr.clone();
            async move {
                println!("🔁 Node {}: Forwarding search to {}", my_id, addr);
                client
                    .search_by_prefix(&prefix, &addr)
                    .await
                    .unwrap_or_else(|e| {
                        eprintln!("❌ Node {}: Error searching on {}: {}", my_id, addr, e);
                        vec![]
                    })
            }
        });

    let mut all_results = join_all(remote_tasks).await;
    all_results.push(local_result);

    let combined: Vec<KeyValue> = all_results.into_iter().flatten().collect();

    println!("✅ Node {}: Returning total of {} matches", my_id, combined.len());

    Json(combined)
}