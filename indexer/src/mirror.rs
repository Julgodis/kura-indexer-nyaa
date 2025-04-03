use std::{path::PathBuf, str::FromStr};

use axum::{
    Extension, Json,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
};

use anyhow::Result;
use reqwest::Url;

use crate::{client::ListQuery, request_tracker::RequestTracker, ClientExt};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorListRequest {
    #[serde(default)]
    #[serde(rename = "p")]
    pub page: Option<usize>,
    #[serde(default)]
    #[serde(rename = "c")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(rename = "s")]
    pub sort: Option<String>,
    #[serde(default)]
    #[serde(rename = "o")]
    pub order: Option<String>,
    #[serde(default)]
    #[serde(rename = "f")]
    pub filter: Option<String>,
    #[serde(default)]
    #[serde(rename = "q")]
    pub query: Option<String>,
}

impl MirrorListRequest {
    pub fn validate(self) -> Self {
        let page = match self.page {
            Some(page) if page > 0 => Some(page),
            None => None,
            _ => {
                tracing::warn!("invalid page number, defaulting to 1");
                Some(1)
            }
        };
        let category = self.category;
        let sort = match self.sort {
            Some(sort)
                if ["id", "size", "seeders", "leechers", "downloads", "comments"]
                    .contains(&sort.as_str()) =>
            {
                Some(sort)
            }
            None => None,
            _ => {
                tracing::warn!("invalid sort option, defaulting to None");
                None
            }
        };
        let order = match self.order {
            Some(order) if ["asc", "desc"].contains(&order.as_str()) => Some(order),
            None => None,
            _ => {
                tracing::warn!("invalid order option, defaulting to None");
                None
            }
        };
        let filter = match self.filter {
            Some(filter) if ["0", "1", "2"].contains(&filter.as_str()) => Some(filter),
            None => None,
            _ => {
                tracing::warn!("invalid filter option, defaulting to None");
                None
            }
        };
        let query = match self.query {
            Some(query) if !query.is_empty() => Some(query),
            None => None,
            _ => {
                tracing::warn!("invalid query, defaulting to None");
                None
            }
        };
        Self {
            page,
            category,
            sort,
            order,
            filter,
            query,
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorListResponse {
    pub items: Vec<MirrorListItem>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorListItem {
    pub id: usize,
    pub title: String,
    pub pub_date: chrono::DateTime<chrono::Utc>,
    pub description: Option<String>,
    pub category: String,
    pub size: u64,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub comments: usize,
    pub trusted: bool,
    pub remake: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorViewComment {
    pub id: usize,
    pub user: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub edited_date: Option<chrono::DateTime<chrono::Utc>>,
    pub content: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorViewFile {
    pub id: usize,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorViewResponse {
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
    pub comments: Vec<MirrorViewComment>,
    pub files: Vec<MirrorViewFile>,
}

#[axum::debug_handler]
async fn mirror_list_handler(
    Extension(client): Extension<ClientExt>,
    Query(request): Query<MirrorListRequest>,
) -> impl IntoResponse {
    let request = request.validate();
    let query = ListQuery {
        page: request.page,
        category: request.category,
        sort: request.sort,
        order: request.order,
        filter: request.filter,
        query: request.query,
    }
    .remove_defaults();

    match client.lock().await.list(&query).await {
        Ok(items) => {
            let items = items
                .iter()
                .map(|item| MirrorListItem {
                    id: item.id,
                    title: item.title.clone(),
                    pub_date: item.pub_date,
                    description: item.description.clone(),
                    category: item.category.clone(),
                    size: item.size,
                    seeders: item.seeders,
                    leechers: item.leechers,
                    downloads: item.downloads,
                    comments: item.comments,
                    trusted: item.trusted,
                    remake: item.remake,
                })
                .collect::<Vec<_>>();

            let response = MirrorListResponse { items };

            Json(response).into_response()
        }
        Err(err) => {
            tracing::error!("failed to fetch mirror list: {:?}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch mirror list: {:?}", err),
            )
                .into_response()
        }
    }
}

#[axum::debug_handler]
async fn mirror_view_handler(
    Extension(client): Extension<ClientExt>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match client.lock().await.view(&id).await {
        Ok(item) => {
            let response = MirrorViewResponse {
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
                    .map(|comment| MirrorViewComment {
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
                    .map(|file| MirrorViewFile {
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
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch mirror view: {:?}", err),
            )
                .into_response()
        }
    }
}

#[axum::debug_handler]
async fn mirror_download_handler(
    Extension(client): Extension<ClientExt>,
    Path(id): Path<String>,
) -> String {
    // Perform the necessary operations to get the mirror download
    // For example, you might want to fetch data from a database or an API

    // Return the HTML response
    format!(
        "<html><body><h1>Mirror Download</h1><p>ID: {}</p></body></html>",
        id
    )
}

#[axum::debug_handler]
async fn mirror_requests_handler(
    Extension(tracker): Extension<RequestTracker>,
) -> impl IntoResponse {
    let requests = tracker.get_requests();
    Json(requests).into_response()
}

pub fn routes(app: axum::Router, enabled: bool) -> axum::Router {
    if enabled {
        app.route("/mirror/list", get(mirror_list_handler))
            .route("/mirror/requests", get(mirror_requests_handler))
            .route("/mirror/view/{id}", get(mirror_view_handler))
            .route("/mirror/download/{id}", get(mirror_download_handler))
    } else {
        app
    }
}
