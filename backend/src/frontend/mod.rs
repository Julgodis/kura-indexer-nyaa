use api::{torrent_handler, torrents_handler};
use kura_indexer::server::ServerBuilderIndexer;
use static_files::{Asset, index_handler, static_handler};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::indexer::NyaaIndexer;

pub mod api;
pub mod static_files;
pub mod types;

pub fn routes(builder: ServerBuilderIndexer<NyaaIndexer>) -> ServerBuilderIndexer<NyaaIndexer> {
    tracing::info!("Frontend feature is enabled. Serving static files.");

    for file in Asset::iter() {
        tracing::debug!("  {}", file);
    }

    let builder = builder
        .route("/", axum::routing::get(index_handler))
        .route("/api/torrent/{id}", axum::routing::get(torrent_handler))
        .route("/api/torrents", axum::routing::post(torrents_handler))
        .route("/static/{*path}", axum::routing::get(static_handler))
        .route("/{*path}", axum::routing::get(index_handler))
        .with_router(|x| x.layer(TraceLayer::new_for_http()));

    if cfg!(debug_assertions) {
        builder.with_router(|router| router.layer(CorsLayer::permissive()))
    } else {
        builder
    }
}
