use hyper::{Request, StatusCode};
use axum::body::Body;
use axum::body::to_bytes;
use kvstore_api::build_app;
use serde_json::json;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_put_and_get() {
    let app = build_app(); // Función que debes exponer en `main.rs` o `lib.rs`

    // PUT /kv/foo
    let request = Request::builder()
        .method("PUT")
        .uri("/kv/foo")
        .header("Content-Type", "application/json")
        .body(Body::from(json!({ "value": "bar" }).to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // GET /kv/foo
    let request = Request::builder()
        .uri("/kv/foo")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
    assert_eq!(body, r#""bar""#);
}
