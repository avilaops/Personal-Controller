//! Personal Controller API Server

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    // db: Arc<pc_db::PersonalControllerDb>,
    // llm: Arc<pc_llm::PersonalControllerLlm>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("pc_api=debug,axum=debug")
        .init();

    tracing::info!("🚀 Starting Personal Controller API");

    // TODO: Initialize database and LLM
    // let db = pc_db::PersonalControllerDb::connect("http://localhost:8000").await?;
    // let mut llm = pc_llm::PersonalControllerLlm::new(pc_llm::LlmConfig::default())?;
    // llm.initialize().await?;

    let state = AppState {
        // db: Arc::new(db),
        // llm: Arc::new(llm),
    };

    let app = Router::new()
        // Root and health
        .route("/", get(root))
        .route("/health", get(health))

        // Companies API
        .route("/api/v1/companies", get(list_companies).post(create_company))
        .route("/api/v1/companies/:id", get(get_company).put(update_company).delete(delete_company))

        // Freight Orders API
        .route("/api/v1/freight-orders", get(list_freight_orders).post(create_freight_order))
        .route("/api/v1/freight-orders/:id", get(get_freight_order).put(update_freight_order).delete(delete_freight_order))

        // Timesheets API
        .route("/api/v1/timesheets", get(list_timesheets).post(create_timesheet))
        .route("/api/v1/timesheets/:id", get(get_timesheet).put(update_timesheet).delete(delete_timesheet))

        // Routes API
        .route("/api/v1/routes", get(list_routes).post(create_route))
        .route("/api/v1/routes/:id", get(get_route).put(update_route).delete(delete_route))

        // LLM Chat API
        .route("/api/v1/chat", post(chat))
        .route("/api/v1/chat/history", get(chat_history))
        .route("/api/v1/chat/clear", post(clear_chat))

        // Statistics and Analytics
        .route("/api/v1/stats", get(stats))
        .route("/api/v1/stats/companies", get(company_stats))
        .route("/api/v1/stats/freight", get(freight_stats))
        .route("/api/v1/stats/timesheets", get(timesheet_stats))

        // Middleware
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    tracing::info!("🌐 Server listening on http://{}", addr);
    tracing::info!("📚 API Documentation: http://{}/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ===== Root & Health =====

async fn root() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "Personal Controller API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "REST API for Ávila Transportes Personal Controller",
        "status": "online",
        "endpoints": {
            "companies": "/api/v1/companies",
            "freight_orders": "/api/v1/freight-orders",
            "timesheets": "/api/v1/timesheets",
            "routes": "/api/v1/routes",
            "chat": "/api/v1/chat",
            "stats": "/api/v1/stats"
        }
    }))
}

async fn health() -> impl IntoResponse {
    let headers = HeaderMap::new();
    (StatusCode::OK, headers, "OK")
}

// ===== Query Parameters =====

#[derive(Deserialize)]
struct ListQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize,
}

fn default_page() -> usize { 1 }
fn default_per_page() -> usize { 10 }

#[derive(Serialize)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    pagination: Pagination,
}

#[derive(Serialize)]
struct Pagination {
    page: usize,
    per_page: usize,
    total: usize,
    total_pages: usize,
}

// ===== Companies API =====

async fn list_companies(
    State(_state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    tracing::debug!("Listing companies: page={}, per_page={}", query.page, query.per_page);

    // TODO: Query database
    Json(PaginatedResponse {
        data: vec![],
        pagination: Pagination {
            page: query.page,
            per_page: query.per_page,
            total: 0,
            total_pages: 0,
        },
    })
}

async fn create_company(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::debug!("Creating company: {:?}", payload);

    (StatusCode::CREATED, Json(serde_json::json!({
        "id": "new-company-id",
        "message": "Company created successfully"
    })))
}

async fn get_company(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Getting company: id={}", id);

    // TODO: Query database
    Json(serde_json::json!({
        "id": id,
        "name": "Example Company",
        "cnpj": "12345678000199"
    }))
}

async fn update_company(
    State(_state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    tracing::debug!("Updating company: id={}, payload={:?}", id, payload);

    Json(serde_json::json!({
        "id": id,
        "message": "Company updated successfully"
    }))
}

async fn delete_company(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("Deleting company: id={}", id);

    (StatusCode::NO_CONTENT, ())
}

async fn get_company(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": "123",
        "nome": "Avila Transportes"
    }))
}

async fn list_freight_orders(
    State(_state): State<AppState>,
    Query(_query): Query<ListQuery>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "data": [],
        "pagination": {
            "page": 1,
            "per_page": 10,
            "total": 0
        }
    }))
}

async fn create_freight_order(
    State(_state): State<AppState>,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(serde_json::json!({
        "id": "456",
        "message": "Freight order created"
    })))
}

async fn get_freight_order(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": "456",
        "numero": "288415"
    }))
}

#[derive(Deserialize)]
struct ChatRequest {
    query: String,
}

async fn chat(
    State(_state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "response": format!("Resposta da LLM para: {}", payload.query),
        "sources": [],
        "confidence": 0.85
    }))
}

async fn stats(State(_state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "companies": 0,
        "freight_orders": 0,
        "timesheets": 0,
        "contacts": 0,
        "routes": 0,
        "manifests": 0
    }))
}
