use api::{torrent_handler, torrents_handler};
use kura_indexer::server::ServerBuilderIndexer;
use static_files::{Asset, index_handler, static_handler};
use tower_http::cors::CorsLayer;

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
        .route("/index.html", axum::routing::get(index_handler))
        .route("/api/torrents", axum::routing::post(torrents_handler))
        .route("/api/torrent/{id}", axum::routing::get(torrent_handler))
        .route("/{*file}", axum::routing::get(static_handler));

    if cfg!(debug_assertions) {
        builder.with_router(|router| router.layer(CorsLayer::permissive()))
    } else {
        builder
    }
}
