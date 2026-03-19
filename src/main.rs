use axum::{routing::get, Router};
use lazy_static::lazy_static;
use prometheus::{gather, register_int_gauge, Encoder, IntGauge, TextEncoder};
use tokio::runtime::Handle;
use tokio::time::{sleep, Duration};

lazy_static! {
    static ref WORKERS_COUNTER: IntGauge =
        register_int_gauge!("tokio_workers", "Number of tokio alive tasks").unwrap();
    static ref THREADS_COUNTER: IntGauge =
        register_int_gauge!("tokio_threads", "Number of tokio threads").unwrap();
}

//#[tokio::main]
#[tokio::main(worker_threads = 9)]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/spawn-thread", get(spawn_thread))
        .route("/prometheus-metrics", get(prometheus_metrics))
        .route("/plain-metrics", get(plain_metrics));

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

async fn prometheus_metrics() -> String {
    // setting
    let handle = Handle::current();
    WORKERS_COUNTER.set(handle.metrics().num_alive_tasks() as i64);
    THREADS_COUNTER.set(handle.metrics().num_workers() as i64);
    // export
    let encoder = TextEncoder::new();
    let metric_families = gather(); // Автоматически включает process метрики
    let mut buf = Vec::new();
    encoder.encode(&metric_families, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}

async fn plain_metrics() -> String {
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
