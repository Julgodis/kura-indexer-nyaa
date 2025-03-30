use std::time::Duration;

use axum::{Extension, Json, body::Body, extract::Path, response::IntoResponse};
use reqwest::{ClientBuilder, StatusCode, Url};

use crate::{
    NyaaContext, data,
    frontend::types::{Torrent, TorrentListResponse, TorrentResponse},
    html,
    indexer::NyaaMode,
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
        (Some(term), category, filter, sort, sort_order, offset, limit) => {
            tracing::info!(
                "request: term: {:?}, category: {:?}, filter: {:?}, sort: {:?}, offset: {:?}, limit: {:?}",
                term,
                category,
                filter,
                sort,
                offset,
                limit
            );

            let count = 75usize;
            let page = offset.map(|o| 1 + (o / count));

            let category = if matches!(category, Some(data::Category::All)) {
                None
            } else {
                category
            };

            let filter = if matches!(filter, Some(data::Filter::NoFilter)) {
                None
            } else {
                filter
            };

            let sort = if matches!(sort, Some(data::Sort::Date)) {
                None
            } else {
                sort
            };

            let sort_order = if matches!(sort_order, Some(data::SortOrder::Descending)) {
                None
            } else {
                sort_order
            };

            let page = if matches!(page, Some(1)) { None } else { page };

            let link = crate::url::NyaaUrlBuilder::new(&context.base_url)
                .with_page_option(if context.mode == NyaaMode::Rss {
                    Some("rss")
                } else {
                    None
                })
                .with_query(term)
                .with_category_option(category)
                .with_filter_option(filter)
                .with_sort_option(sort)
                .with_order_option(sort_order)
                .with_offset_option(page)
                .build();
            let url = Url::parse(&link);

            let url = match url {
                Ok(url) => url,
                Err(err) => {
                    tracing::error!("Error: {:?}", err);
                    return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
                }
            };

            tracing::debug!("request: {:?}", url);

            match context.client.fetch_list(url).await {
                Ok((response, cached)) => {
                    tracing::trace!("response: {:?}", response);
                    if !cached {
                        match context.add_items(&response) {
                            Ok(_) => {}
                            Err(err) => {
                                tracing::error!("Error: {:?}", err);
                                return (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Internal server error",
                                )
                                    .into_response();
                            }
                        }
                    }

                    Json(TorrentListResponse {
                        torrents: response
                            .into_iter()
                            .map(|item| Torrent::from(item))
                            .collect::<Vec<_>>(),
                        offset: offset.unwrap_or(0),
                        count: count,
                        total: count * 5,
                    })
                    .into_response()
                }
                Err(err) => {
                    tracing::error!("Error: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
            }
        }
    }
}

#[axum::debug_handler]
pub async fn torrent_handler(
    Extension(context): Extension<NyaaContext>,
    Path(request): Path<TorrentRequest>,
) -> impl IntoResponse {
    let link = format!("{}view/{}", context.base_url, request.id);
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
                submitter: response.submitter,
                info_link: response.info_link,
                info_hash: response.info_hash,
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
pub async fn download_handler(
    Extension(context): Extension<NyaaContext>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    tracing::debug!("download request: {:?}", id);

    let url = format!("{}download/{}", context.base_url, id);
    let url = Url::parse(&url);
    let url = match url {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
        }
    };

    let result = context.client.download(url).await;
    match result {
        Ok((data, content_type)) => {
            let body = Body::from(data);
            let headers = [
                (axum::http::header::CONTENT_TYPE, content_type),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", id),
                ),
            ];

            (headers, body).into_response()
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
            tracing::trace!("response: {:?}", response);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
