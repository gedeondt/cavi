mod config;
mod app;
mod shard;
mod remote;

use config::load_config;
use crate::shard::router::ShardRouter;
use tokio::net::TcpListener;
use axum::serve;
use crate::app::build_app;
use std::sync::Arc;
use std::{env};
use crate::remote::HttpNodeClient;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Leer argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <node_id> <config_path>", args[0]);
        std::process::exit(1);
    }

    let node_id: usize = args[1].parse().expect("Invalid node ID");
    let config_path = &args[2];

    // Cargar configuración
    let config = load_config(config_path);
    let shard = config.shards.iter().find(|s| s.id == node_id)
        .expect("Node ID not found in config");

    let router = std::sync::Arc::new(ShardRouter::new(node_id, config.shards.clone()));
    let client = Arc::new(HttpNodeClient::new());

    let app = build_app(router, client);

    let addr: SocketAddr = shard.addr.parse().expect("Invalid socket address");
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!(
        "🟢 Node {} listening on {} (\"{}\" - \"{}\")",
        shard.id, addr, shard.range_start, shard.range_end
    );
    println!("🚀 HTTP server bound at http://{}", addr);

    serve(listener, app).await.unwrap();
}