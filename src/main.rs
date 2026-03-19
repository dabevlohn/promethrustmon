use axum::{routing::get, Router};
use prometheus::{gather, Encoder, TextEncoder};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/metrics", get(metrics_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = gather(); // Автоматически включает process метрики
    let mut buf = Vec::new();
    encoder.encode(&metric_families, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}
