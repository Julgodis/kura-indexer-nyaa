use axum::{response::IntoResponse, Extension, Json};

use crate::{request_tracker, MirrorExt};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct HealthResponse {
    mirrors: Vec<MirrorHealth>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct MirrorHealth {
    id: String,
    name: String,
    requests: Vec<(chrono::DateTime<chrono::Utc>, String, bool, bool, f64)>,
}

#[axum::debug_handler]
pub async fn handler(
    Extension(mext): Extension<MirrorExt>,
    Extension(request_tracker): Extension<request_tracker::RequestTracker>,
) -> impl IntoResponse {
    let mut health = HealthResponse { mirrors: vec![] };

    for mirror in mext.iter() {
        let requests = request_tracker.get_requests(mirror.id());
        health.mirrors.push(MirrorHealth {
            id: mirror.id().into(),
            name: mirror.name().into(),
            requests: requests,
        });
    }

    Json(health).into_response()
}
