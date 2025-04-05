use axum::{Extension, Json, response::IntoResponse};

use crate::{MirrorExt, cli::MirrorType};

pub mod list;
pub mod magnet;
pub mod view;

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
pub async fn handler(Extension(mext): Extension<MirrorExt>) -> impl IntoResponse {
    let items = mext
        .iter()
        .map(|mirror| MirrorSiteItem {
            id: mirror.id().into(),
            name: mirror.name().into(),
            hidden: mirror.is_hidden(),
            ty: mirror.ty(),
        })
        .collect::<Vec<_>>();
    Json(MirrorSiteResponse { items }).into_response()
}
