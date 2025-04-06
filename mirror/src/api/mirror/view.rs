use axum::{Extension, Json, extract::Path, response::IntoResponse};

use crate::MirrorExt;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ViewComment {
    pub id: usize,
    pub user: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub edited_date: Option<chrono::DateTime<chrono::Utc>>,
    pub content: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ViewFile {
    pub id: usize,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ViewResponse {
    pub id: usize,
    pub title: String,
    pub pub_date: chrono::DateTime<chrono::Utc>,
    pub description_md: String,
    pub category: String,
    pub size: u64,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub trusted: bool,
    pub remake: bool,
    pub magnet_link: Option<String>,
    pub comments: Vec<ViewComment>,
    pub files: Vec<ViewFile>,
}

#[axum::debug_handler]
pub async fn handler(
    Extension(mext): Extension<MirrorExt>,
    Path((mirror_id, item_id)): Path<(String, String)>,
) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let Some(mirror) = mext.find_by_id(&mirror_id) else {
        tracing::error!("mirror not found");
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Mirror not found".to_string(),
        )
            .into_response();
    };

    match mirror.client.lock().await.view(&item_id).await {
        Ok(item) => {
            let response = ViewResponse {
                id: item.id,
                title: item.title,
                pub_date: item.pub_date,
                description_md: item.description_md,
                category: item.category,
                size: item.size,
                seeders: item.seeders,
                leechers: item.leechers,
                downloads: item.downloads,
                trusted: item.trusted,
                remake: item.remake,
                magnet_link: item.magnet_link,
                comments: item
                    .comments
                    .iter()
                    .map(|comment| ViewComment {
                        id: comment.id,
                        user: comment.user.clone(),
                        date: comment.date,
                        edited_date: comment.edited_date,
                        content: comment.content.clone(),
                        avatar: comment.avatar.clone(),
                    })
                    .collect(),
                files: item
                    .files
                    .iter()
                    .map(|file| ViewFile {
                        id: file.id,
                        name: file.name.clone(),
                        size: file.size,
                    })
                    .collect(),
            };

            Json(response).into_response()
        }
        Err(err) => {
            tracing::error!("failed to fetch mirror view: {:?}", err);
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to fetch mirror view: {:?}", err),
            )
                .into_response()
        }
    }
}
