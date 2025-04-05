use axum::{
    Extension, Json,
    extract::{Path, Query},
    response::IntoResponse,
};

use crate::{MirrorExt, client::ListQuery};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ListItem {
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
pub struct ListRequest {
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

impl ListRequest {
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
pub struct ListResponse {
    pub items: Vec<ListItem>,
}

#[axum::debug_handler]
pub async fn handler(
    Extension(mext): Extension<MirrorExt>,
    Path(mirror_id): Path<String>,
    Query(request): Query<ListRequest>,
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

    let Some(mirror) = mext.find_by_id(&mirror_id) else {
        tracing::error!("mirror not found");
        return (
            axum::http::StatusCode::BAD_REQUEST,
            "Mirror not found".to_string(),
        )
            .into_response();
    };

    match mirror.client.lock().await.list(&query).await {
        Ok(items) => {
            let items = items
                .iter()
                .map(|item| ListItem {
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

            let response = ListResponse { items };
            Json(response).into_response()
        }
        Err(err) => {
            tracing::error!("failed to fetch mirror list: {:?}", err);
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Failed to fetch mirror list: {:?}", err),
            )
                .into_response()
        }
    }
}
