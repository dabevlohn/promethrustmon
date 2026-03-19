use axum::{routing::get, Router};
use prometheus::{gather, Encoder, TextEncoder};
use tokio::runtime::Handle;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/spawn-thread", get(spawn_thread))
        .route("/process-metrics", get(process_metrics))
        .route("/tokio-metrics", get(tmetrics));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9090").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn spawn_thread() -> String {
    tokio::spawn(async {
        println!("new thread started");
        factorial_iter(15);
        sleep(Duration::from_secs(10)).await;
        println!("new thread dropped");
    });
    "OK".to_string()
}

async fn process_metrics() -> String {
    let encoder = TextEncoder::new();

    let metric_families = gather(); // Автоматически включает process метрики
    let mut buf = Vec::new();
    encoder.encode(&metric_families, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}

async fn tmetrics() -> String {
    let handle = Handle::current();
    let m = format!(
        "{:?} {:?}",
        handle.metrics().num_alive_tasks(),
        handle.metrics().num_workers()
    );
    m.to_string()
}

fn factorial_iter(n: u64) -> u64 {
    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    result
}
