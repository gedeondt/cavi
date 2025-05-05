# 📦 CAVI — Distributed Key-Value Store (Learning Project)

**CAVI** is a personal, educational project exploring how to build a lightweight and performant distributed key-value service in [Rust](https://www.rust-lang.org/).

The goal is not to replicate full production-grade databases, but to **learn, experiment, and understand** distributed systems design, safe concurrency, and systems-level performance using modern Rust idioms.

---

## 🚀 Features (so far)

- ✅ **In-memory key-value store** with basic CRUD (`GET`, `PUT`, `DELETE`)
- ✅ **Search by prefix** with optional fan-out to all nodes
- ✅ **Multi-shard support** based on lexicographic key ranges
- ✅ **Forwarding logic**: when a node receives a request for a key outside its range, it proxies it to the appropriate node
- ✅ **Configurable shard layout** via a simple YAML file
- ✅ **Basic HTTP API** using [Axum](https://github.com/tokio-rs/axum)
- ✅ **Shared types and testable interfaces** for future extensibility
- ✅ **Request loop prevention** via custom headers for cross-node communication

---

## 📂 Example layout

```yaml
# config.yaml
shards:
  - id: 0
    addr: 127.0.0.1:3100
    range_start: "a"
    range_end: "m"
  - id: 1
    addr: 127.0.0.1:3101
    range_start: "n"
    range_end: "z"
```

---

## 🛠 Run it locally

Start two nodes:

```bash
cargo run --bin kvstore-api -- 0 ./config.yaml
cargo run --bin kvstore-api -- 1 ./config.yaml
```

Use Postman or `curl` to test endpoints like:

- `GET /kv/foo`
- `PUT /kv/foo`
- `DELETE /kv/foo`
- `GET /search?prefix=f`

---

## 🎯 What’s next?

This is a learning-driven project, so features evolve as new concepts are explored. Potential future additions:

- 🔒 Durable backends (file or embedded DB)
- 🌐 gRPC-based cross-node messaging
- ♻️ Gossip or ring-based shard discovery
- 📊 Telemetry and diagnostics
- 📦 CLI client

---

## 🙌 Contributions

This is a personal playground project. Ideas, issues, and discussions are always welcome — but please keep in mind the project's educational nature.
