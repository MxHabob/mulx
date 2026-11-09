use axum::{Json, Router, http::StatusCode, routing::get};
use mulx_app::CoreDomainService;
use mulx_infrastructure::RuntimeStatus;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Debug, Serialize)]
struct ApiEnvelope<T> {
    ok: bool,
    data: T,
}

#[derive(Debug, Serialize)]
struct HealthPayload {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct ReadinessPayload {
    ready: bool,
    database: bool,
    storage: bool,
}

async fn health() -> (StatusCode, Json<ApiEnvelope<HealthPayload>>) {
    let summary = CoreDomainService::health_check();

    (
        StatusCode::OK,
        Json(ApiEnvelope {
            ok: true,
            data: HealthPayload {
                status: "ok",
                service: summary.service,
                version: summary.version,
            },
        }),
    )
}

async fn readiness() -> (StatusCode, Json<ApiEnvelope<ReadinessPayload>>) {
    let runtime = RuntimeStatus::ready();

    (
        StatusCode::OK,
        Json(ApiEnvelope {
            ok: true,
            data: ReadinessPayload {
                ready: runtime.database_ready && runtime.storage_ready,
                database: runtime.database_ready,
                storage: runtime.storage_ready,
            },
        }),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/ready", get(readiness));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("MULX API listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind API listener");

    axum::serve(listener, app).await.expect("server to run");
}
