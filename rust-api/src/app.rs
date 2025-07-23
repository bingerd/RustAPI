use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing::post,
    Router,
    body::Body,
    http::{StatusCode, header::{CONTENT_TYPE, HeaderValue}}
};
use serde::Deserialize;
use std::{sync::Arc};
use tch::{CModule};

pub struct AppState {
    pub model: CModule,
}

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(homepage))
        .route("/api/v1/ping", post(ping))
        .route("/api/v1/recommend", post(recommend))
        .with_state(state)
}

async fn homepage() -> impl IntoResponse {
    Html("<a href='/api/v1/ping'>ping</a>")
}

async fn ping() -> impl IntoResponse {
    "pong"
}

#[derive(Deserialize)]
struct RecommendQuery {
    user_id: String,
    top_k: Option<usize>,
}

// Dummy function
async fn recommend(
    _state: axum::extract::State<Arc<AppState>>,
    Query(params): Query<RecommendQuery>,
) -> impl IntoResponse {
    let top_k = params.top_k.unwrap_or(10);
    if top_k == 0 {
        return (axum::http::StatusCode::BAD_REQUEST, "Illegal value for top_k").into_response();
    }

    // Placeholder: Replace with actual predict_groups logic
    let groups: Vec<String> = (0..top_k).map(|i| format!("group_{}", i)).collect();
    let scores: Vec<f32> = (0..top_k).map(|i| 1.0 - (i as f32 / top_k as f32)).collect();

    let mut csv = "group_id,score\n".to_string();
    for (g, s) in groups.iter().zip(scores.iter()) {
        csv.push_str(&format!("{},{}\n", g, s));
    }


    let mut response = Response::new(Body::from(csv));
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/csv"),
    );

    response
}
