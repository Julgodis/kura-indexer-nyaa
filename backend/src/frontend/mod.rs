use api::{torrent_handler, torrents_handler};
use kura_indexer::server::ServerBuilderIndexer;
use static_files::{Asset, index_handler, static_handler};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

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
        .route(
            "/api/stats/torrents-per-day",
            axum::routing::get(api::stats_torrents_per_day_handler),
        )
        .route("/api/stats/actions", axum::routing::get(api::stats_events))
        .route("/download/{id}", axum::routing::get(api::download_handler))
        .route("/static/{*path}", axum::routing::get(static_handler))
        .route("/{*path}", axum::routing::get(index_handler))
        .with_router(|x| {
            x.layer(
                CompressionLayer::new()
                    .br(true)
                    .deflate(true)
                    .gzip(true)
                    .zstd(true),
            )
        });

    if cfg!(debug_assertions) {
        builder.with_router(|router| router.layer(CorsLayer::permissive()))
    } else {
        builder
    }
}
