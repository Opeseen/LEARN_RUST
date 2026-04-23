use crate::utility::{ApiResponse, constants};
use axum::{
    Json,
    http::{HeaderMap, StatusCode},
};
use uuid::Uuid;

pub fn extract_org_id(headers: &HeaderMap) -> Result<Uuid, (StatusCode, Json<ApiResponse<()>>)> {
    headers
        .get("x-organization-id")
        .and_then(|h_value| h_value.to_str().ok())
        .and_then(|input| Uuid::parse_str(input).ok())
        .ok_or_else(|| ApiResponse::error(StatusCode::BAD_REQUEST, constants::MISSING_ORG_HEADER))
}
