use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use parking_lot::Mutex;
use ort::{session::Session, value::Value};
use ndarray::array;
use tokio::task;
use serde_json::json;
use http::StatusCode;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RegressQuery {
    pub input: f32,
}

// --- Session pool ---
pub struct SessionPool {
    sessions: Vec<Arc<Mutex<Session>>>,
    counter: AtomicUsize,
}

impl SessionPool {
    pub fn new(sessions: Vec<Session>) -> Self {
        let sessions = sessions.into_iter().map(|s| Arc::new(Mutex::new(s))).collect();
        Self {
            sessions,
            counter: AtomicUsize::new(0),
        }
    }

    pub fn acquire(&self) -> Arc<Mutex<Session>> {
        let idx = self.counter.fetch_add(1, Ordering::Relaxed) % self.sessions.len();
        self.sessions[idx].clone()
    }
}

// --- Create router ---
pub fn create_app(pool: Arc<SessionPool>) -> Router {
    Router::new()
        .route("/api/v1/regress", post(move |json| regress(json, pool.clone())))
}

// --- Regression handler ---
async fn regress(
    Json(params): Json<RegressQuery>,
    pool: Arc<SessionPool>,
) -> impl IntoResponse {
    let prediction = task::spawn_blocking(move || {
        let session = pool.acquire();
        let mut session = session.lock();

        // 2D array [1,1] as expected by ONNX
        // println!("params.input {}", params.input);
        let input_array = array![[params.input]];
        // println!("input_array {}", input_array);
        let mut input_map = std::collections::HashMap::new();
        // println!("input_map {}", input_map);
        input_map.insert("input", Value::from_array(input_array)?);
        // println!("input_map_inserted {}", input_map);

        let outputs = session.run(input_map)?;
        let arr = outputs[0].try_extract_array::<f32>()?;
        // println!("arr {}", arr[[0,0]]);
        Ok::<f32, ort::Error>(arr[[0,0]])
    })
    .await
    .unwrap_or_else(|e| Err(ort::Error::new(e.to_string())));

    match prediction {
        Ok(pred) => {
            let body = json!({
                "input": params.input,
                "prediction": pred
            });
            (StatusCode::OK, axum::Json(body))
        }
        Err(e) => {
            let body = json!({
                "error": format!("Inference failed: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(body))
        }
    }
}
