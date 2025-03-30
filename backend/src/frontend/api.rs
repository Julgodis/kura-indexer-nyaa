use std::time::Duration;

use axum::{Extension, Json, extract::Path, response::IntoResponse};
use reqwest::{ClientBuilder, StatusCode, Url};

use crate::{
    NyaaContext, data,
    frontend::types::{Torrent, TorrentListResponse, TorrentResponse},
    html,
};

use super::types::{TorrentListRequest, TorrentRequest};

#[axum::debug_handler]
pub async fn torrents_handler(
    Extension(context): Extension<NyaaContext>,
    Json(request): Json<TorrentListRequest>,
) -> impl IntoResponse {
    let term = if let Some(term) = request.term {
        if term.is_empty() { None } else { Some(term) }
    } else {
        None
    };

    match (
        term,
        request.category,
        request.filter,
        request.sort,
        request.sort_order,
        request.offset,
        request.limit,
    ) {
        (None, category, filter, sort, sort_order, offset, limit) => {
            tracing::debug!("request: offset: {:?}, limit: {:?}", offset, limit);

            let response: anyhow::Result<_> = (|| {
                let db = context.db()?;
                let item_count = NyaaContext::get_item_count(&db)?;
                let (offset, count, items) = NyaaContext::get_items(
                    &db, offset, limit, None, category, filter, sort, sort_order,
                )?;

                Ok(TorrentListResponse {
                    torrents: items
                        .into_iter()
                        .map(|item| Torrent::from(item))
                        .collect::<Vec<_>>(),
                    offset,
                    count,
                    total: item_count,
                })
            })();

            match response {
                Ok(response) => {
                    tracing::trace!("response: {:?}", response);
                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(err) => {
                    tracing::error!("Error: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
            }
        }
        (term, category, filter, sort, sort_order, offset, limit) => {
            tracing::info!(
                "request: term: {:?}, category: {:?}, filter: {:?}, sort: {:?}, offset: {:?}, limit: {:?}",
                term,
                category,
                filter,
                sort,
                offset,
                limit
            );
            (StatusCode::BAD_REQUEST, "Invalid request").into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn torrent_handler(
    Extension(context): Extension<NyaaContext>,
    Path(request): Path<TorrentRequest>,
) -> impl IntoResponse {
    let link = format!("https://nyaa.si/view/{}", request.id);
    let url = Url::parse(&link);

    let url = match url {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
        }
    };

    match context.client.fetch_view(url).await {
        Ok(response) => {
            tracing::trace!("response: {:?}", response);
            let torrent = TorrentResponse {
                guid: response.id,
                title: response.title,
                link: response.link,
                info_hash: "".to_string(),
                pub_date: response.date.to_rfc2822(),
                seeders: response.seeders,
                leechers: response.leechers,
                downloads: response.downloads,
                category_id: response.category.to_string(),
                category: response.category.english(),
                size: response.size,
                trusted: response.trusted,
                remake: response.remake,
                download_link: response.download_link,
                magnet_link: response.magnet_link,
                description: markdown::to_html(&response.description),
                description_markdown: response.description,
                files: response.files,
                comments: response.comments,
            };
            (StatusCode::OK, Json(torrent)).into_response()
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn stats_torrents_per_day_handler(
    Extension(context): Extension<NyaaContext>,
) -> impl IntoResponse {
    let result: anyhow::Result<_> = (|| {
        let db = context.db()?;
        Ok(NyaaContext::get_torrent_per_day(&db)?)
    })();

    match result {
        Ok(response) => {
            tracing::trace!("response: {:?}", response);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn stats_events(Extension(context): Extension<NyaaContext>) -> impl IntoResponse {
    let result: anyhow::Result<_> = (|| {
        let event_db = context.event_db()?;
        Ok(NyaaContext::get_events(&event_db)?)
    })();

    match result {
        Ok(response) => {
            tracing::debug!("response: {:?}", response);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
