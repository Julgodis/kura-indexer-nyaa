use std::path::PathBuf;

use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
};
use clap::Parser;
use cli::{MirrorConfig, MirrorType};
use reqwest::{StatusCode, Url};
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod cli;

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
    pub comments: Vec<MirrorViewComment>,
    pub files: Vec<MirrorViewFile>,
    pub magnet_link: Option<String>,
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MagnetResponse {
    pub magnet_link: String,
}

#[axum::debug_handler]
async fn list_handler(
    Path(mirror_id): Path<String>,
    Extension(Mirrors(mirrors)): Extension<Mirrors>,
    Query(request): axum::extract::Query<MirrorListRequest>,
    Extension(client): Extension<reqwest::Client>,
) -> impl IntoResponse {
    let mirror = mirrors
        .iter()
        .find(|mirror| mirror.id == mirror_id)
        .and_then(|mirror| mirror.api_url_parsed.clone())
        .clone();
    let Some(mirror) = mirror else {
        tracing::warn!("mirror not found");
        return (
            StatusCode::NOT_FOUND,
            Json(MirrorListResponse { items: vec![] }),
        )
            .into_response();
    };

    let Ok(url) = mirror.join("/mirror/list") else {
        tracing::warn!("failed to parse URL");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MirrorListResponse { items: vec![] }),
        )
            .into_response();
    };

    tracing::trace!("mirror url: {:?}", url);
    let Ok(inner_request) = client.get(url).query(&request).send().await else {
        tracing::warn!("failed to send request");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MirrorListResponse { items: vec![] }),
        )
            .into_response();
    };

    let inner_response = match inner_request.json::<MirrorListResponse>().await {
        Ok(inner_response) => inner_response,
        Err(_) => {
            tracing::warn!("failed to parse response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MirrorListResponse { items: vec![] }),
            )
                .into_response();
        }
    };

    Json(inner_response).into_response()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorSiteResponse {
    pub items: Vec<MirrorSiteItem>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MirrorSiteItem {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    #[serde(rename = "type")]
    pub ty: MirrorType,
}

#[axum::debug_handler]
async fn mirror_handler(Extension(Mirrors(mirrors)): Extension<Mirrors>) -> impl IntoResponse {
    let items = mirrors
        .iter()
        .map(|mirror| MirrorSiteItem {
            id: mirror.id.clone(),
            name: mirror.name.clone(),
            hidden: mirror.hidden.unwrap_or(false),
            ty: mirror.ty.clone(),
        })
        .collect::<Vec<_>>();
    Json(MirrorSiteResponse { items }).into_response()
}

#[axum::debug_handler]
async fn magnet_handler(
    Path((mirror_id, item_id)): Path<(String, String)>,
    Extension(Mirrors(mirrors)): Extension<Mirrors>,
    Extension(client): Extension<reqwest::Client>,
) -> impl IntoResponse {
    let mirror = mirrors
        .iter()
        .find(|mirror| mirror.id == mirror_id)
        .and_then(|mirror| mirror.api_url_parsed.clone())
        .clone();
    let Some(mirror) = mirror else {
        tracing::warn!("mirror not found");
        return (
            StatusCode::NOT_FOUND,
            Json(MagnetResponse {
                magnet_link: "".to_string(),
            }),
        )
            .into_response();
    };

    let Ok(url) = mirror.join(&format!("/mirror/view/{}", item_id)) else {
        tracing::warn!("failed to parse URL");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MagnetResponse {
                magnet_link: "".to_string(),
            }),
        )
            .into_response();
    };

    tracing::trace!("mirror url: {:?}", url);
    let Ok(inner_request) = client.get(url).send().await else {
        tracing::warn!("failed to send request");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MagnetResponse {
                magnet_link: "".to_string(),
            }),
        )
            .into_response();
    };

    let inner_response = match inner_request.json::<MirrorViewResponse>().await {
        Ok(inner_response) => {
            if let Some(magnet_link) = inner_response.magnet_link {
                MagnetResponse { magnet_link }
            } else {
                tracing::warn!("magnet link not found");
                return (
                    StatusCode::NOT_FOUND,
                    Json(MagnetResponse {
                        magnet_link: "".to_string(),
                    }),
                )
                    .into_response();
            }
        }
        Err(_) => {
            tracing::warn!("failed to parse response");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MagnetResponse {
                    magnet_link: "".to_string(),
                }),
            )
                .into_response();
        }
    };

    Json(inner_response).into_response()
}

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
async fn health_handler(
    Extension(client): Extension<reqwest::Client>,
    Extension(Mirrors(mirrors)): Extension<Mirrors>,
) -> impl IntoResponse {
    let mut health = HealthResponse { mirrors: vec![] };

    for mirror in mirrors.iter() {
        let url = match mirror.api_url_parsed.clone() {
            Some(url) => url,
            None => {
                tracing::warn!("failed to parse URL");
                continue;
            }
        };

        let url = match url.join("/mirror/requests") {
            Ok(url) => url,
            Err(_) => {
                tracing::warn!("failed to parse URL");
                continue;
            }
        };

        let Ok(inner_request) = client.get(url).send().await else {
            tracing::warn!("failed to send request");
            continue;
        };

        let inner_response = match inner_request
            .json::<Vec<(chrono::DateTime<chrono::Utc>, String, bool, bool, f64)>>()
            .await
        {
            Ok(inner_response) => inner_response,
            Err(_) => {
                tracing::warn!("failed to parse response");
                continue;
            }
        };

        health.mirrors.push(MirrorHealth {
            id: mirror.id.clone(),
            name: mirror.name.clone(),
            requests: inner_response,
        });
    }

    Json(health).into_response()
}

#[derive(Debug, Clone)]
struct Mirrors(Vec<MirrorConfig>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::Cli::parse();
    let mut config = cli::load_config(&cli.config)?;
    tracing::info!("loaded config: {:?}", config);

    let index_path = config.static_dir.join("index.html");

    for mirror in &mut config.mirror {
        let api_url = if let Ok(url) = Url::parse(&mirror.api_url) {
            url
        } else if let Ok(url) = Url::parse(&format!("http://{}", mirror.api_url)) {
            url
        } else {
            Url::parse(&mirror.api_url)?
        };

        mirror.api_url_parsed = Some(api_url.clone());
    }

    let mut app = axum::Router::new()
        .route_service("/", ServeFile::new(index_path.clone()))
        .nest_service(
            "/api",
            Router::new()
            .route("/mirror/{mirror}/list", axum::routing::get(list_handler))
            .route(
                "/mirror/{mirror}/magnet/{id}",
                axum::routing::get(magnet_handler),
            )
            .route("/mirror", axum::routing::get(mirror_handler))
            .route("/health", axum::routing::get(health_handler)),
        )
        .route_service("/{*path}", ServeFile::new(index_path))
        .nest_service("/static", ServeDir::new(config.static_dir))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    if config.cors_allow_everyone.unwrap_or(false) {
        app = app.layer(tower_http::cors::CorsLayer::very_permissive());
    }

    let client = reqwest::Client::builder()
        .user_agent("Nyaa-Mirror/0.1")
        .build()?;

    let app = app
        .layer(Extension(client))
        .layer(Extension(Mirrors(config.mirror.clone())));

    let listener = tokio::net::TcpListener::bind(config.listen_addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
