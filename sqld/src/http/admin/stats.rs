use std::sync::Arc;

use serde::Serialize;

use axum::extract::{Path, State};
use axum::Json;

use crate::namespace::{MakeNamespace, NamespaceName};
use crate::replication::FrameNo;
use crate::stats::Stats;

use super::AppState;

#[derive(Serialize)]
pub struct StatsResponse {
    pub rows_read_count: u64,
    pub rows_written_count: u64,
    pub storage_bytes_used: u64,
    pub write_requests_delegated: u64,
    pub current_frame_no: FrameNo,
}

impl From<&Stats> for StatsResponse {
    fn from(stats: &Stats) -> Self {
        Self {
            rows_read_count: stats.rows_read(),
            rows_written_count: stats.rows_written(),
            storage_bytes_used: stats.storage_bytes_used(),
            write_requests_delegated: stats.write_requests_delegated(),
            current_frame_no: stats.get_current_frame_no(),
        }
    }
}

impl From<Stats> for StatsResponse {
    fn from(stats: Stats) -> Self {
        (&stats).into()
    }
}

pub(super) async fn handle_stats<M: MakeNamespace>(
    State(app_state): State<Arc<AppState<M>>>,
    Path(namespace): Path<String>,
) -> crate::Result<Json<StatsResponse>> {
    let stats = app_state
        .namespaces
        .stats(NamespaceName::from_string(namespace)?)
        .await?;
    let resp: StatsResponse = stats.as_ref().into();

    Ok(Json(resp))
}
