use axum::{Extension, Json, extract::Path, http, response::IntoResponse};

use crate::MirrorExt;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MagnetResponse {
    pub magnet_link: String,
}

#[axum::debug_handler]
pub async fn handler(
    Extension(mext): Extension<MirrorExt>,
    Path((mirror_id, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let Some(mirror) = mext.find_by_id(&mirror_id) else {
        tracing::error!("mirror not found");
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Mirror not found".to_string(),
        )
            .into_response();
    };

    match mirror.client.lock().await.magnet_link(&item_id).await {
        Ok(magnet_link) => {
            let response = MagnetResponse { magnet_link };
            Json(response).into_response()
        }
        Err(err) => {
            tracing::error!("Error fetching magnet link: {}", err);
            (http::StatusCode::BAD_REQUEST, "Internal Server Error").into_response()
        }
    }
}
