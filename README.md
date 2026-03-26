# План воркшопа "Мониторинг многопоточного Rust: Prometheus + Promethrustmon"

## 🎬 ** Введение **

### 1. Зачем мониторинг

```
🚨 Production проблемы БЕЗ мониторинга:
    - Threads растут → OOM kill
    - Tokio workers = 64 → CPU 100%
    - Latency P95 = 5s → пользователи уходят
    - Memory leak → $$$ на облаке

✅ С мониторингом:
Grafana alert → PagerDuty → fix за 5 мин
```

### 2. Ключевые метрики

```
🔥 Process: threads, CPU, memory, FDs
⚙️ Tokio: worker_threads, active_tasks, idle_workers
🌐 HTTP: requests/sec, P95 latency, error_rate
```

### 3. Форматы метрик

```
📈 Prometheus (pull модель):
    - Text format `/metrics`
    - Timeseries в Grafana/Prometheus
    - PromQL: rate(http_requests[5m])

🆚 Push (Graphite) ❌ — single point of failure
```

## 💻 ** Live Coding **

### Этап 1: Setup

```bash
# Каждый студент:
git clone https://github.com/dabevlohn/promethrustmon
cd promethrustmon
just r
```

### Этап 2: Базовый сервер

```rust
// students/01-baseline/src/main.rs
#[tokio::main(worker_threads = 16)]  // ❌ Плохо!
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/stress", get(stress));

    // curl localhost:3000/metrics → пусто!
}
```

**Задача**: Запустить → посмотреть `process_threads` в `top`

### Этап 3: Добавляем метрики

```rust
// students/02-monitoring/src/main.rs
use promethrustmon::PrometheusMonitorLayer;

let app = Router::new()
    .route("/", get(root))
    .route("/stress", get(stress))
    .layer(PrometheusMonitorLayer::default());  // ✅ 1 строка!

// curl /metrics → 50+ метрик!
```

**Live demo в Grafana**:

```
process_threads → 20
tokio_worker_threads → 16
http_requests_total{method="GET"} → растёт
```

### Этап 4: Prometheus scrape

**prometheus.yml** (уже настроен):

```yaml
scrape_configs:
  - job_name: "rust-app"
    static_configs: [{ targets: ["app:3000"] }]
```

**Grafana** (http://localhost:3001):

```
• Dashboard "Rust App Overview"
• Query: rate(http_requests_total[2m])
```

## 🔍 ** Профилирование и оптимизация **

### Эксперимент 1: Thread explosion

```rust
// Плохо:
for _ in 0..1000 {
    tokio::spawn(async { heavy_work().await });
}
// process_threads → 1016 ❌ Memory OOM!
```

**Анализ в Grafana**: `process_threads` растёт → alert!

### Эксперимент 2: Tokio tuning

```rust
// До:
#[tokio::main(worker_threads = 16)]

// После:
#[tokio::main(worker_threads = num_cpus::get())]  // 8 ✅
```

**Grafana**: `tokio_idle_workers` = 0 → идеально!

### Эксперимент 3: Load testing

```bash
# Студенты запускают параллельно:
wrk -t16 -c400 -d120s http://localhost:3000/stress

# В Grafana:
http_request_duration_seconds P95 → 500ms 😱
Найти bottleneck → оптимизировать
```

**Flamegraph бонус**:

```bash
cargo flamegraph --release --bin stress_test
```

## 🎯 ** Итоги **

### Что освоили:

```
✅ 1 строка кода → 50+ production метрик
✅ Grafana dashboard для Rust app
✅ Thread tuning по метрикам
✅ Load testing + bottleneck detection
✅ Prometheus scrape config
```

### Production checklist:

```
☑️ process_* metrics (threads, memory)
☑️ tokio_* metrics (workers, tasks)
☑️ http_* metrics (latency P95, error rate)
☑️ Docker + Prometheus + Grafana
☑️ Alerts на Grafana
```

## ❓ ** Q&A и обсуждение **

**Типичные вопросы:**

```
Q: Сколько worker_threads ставить?
A: num_cpus() для IO, 2x для CPU

Q: Как мониторить memory leaks?
A: process_virtual_memory_bytes + alert > 80%

Q: Push или Pull модель?
A: Pull (Prometheus) — де-факто стандарт
```

**Домашка**:

```
1. Добавить promethrustmon в свой проект
2. Grafana dashboard + скриншот
3. PR в свой репозиторий
```
