use crate::organizations::{
    CreateOrganizationDto,
    repository::{OrganizationRepository, OrganizationStorage},
};
use crate::routes::AppState;
use crate::utility::{ApiError, ApiResponse, constants};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn create_organization(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrganizationDto>,
) -> Result<impl IntoResponse, ApiError> {
    let mut conn = state.db.acquire().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    let organization_id = OrganizationStorage::create(&mut *conn, &payload)
        .await
        .map_err(|_| {
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
        })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            organization_id,
            constants::CREATED,
            StatusCode::CREATED,
        )),
    ))
}

pub async fn get_all_organization(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let mut conn = state.db.acquire().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;
    let organizations = OrganizationStorage::find_all(&mut *conn)
        .await
        .map_err(|_| {
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
        })?;

    Ok(Json(ApiResponse::success(
        organizations,
        constants::SUCCESS,
        StatusCode::OK,
    )))
}

pub async fn get_organization(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let mut conn = state.db.acquire().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;
    let organizations = OrganizationStorage::find_by_id(&mut *conn, id)
        .await
        .map_err(|_| {
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
        })?;

    match organizations {
        Some(og) => Ok(Json(ApiResponse::success(
            og,
            constants::SUCCESS,
            StatusCode::OK,
        ))),
        None => Err(ApiResponse::error(
            StatusCode::NOT_FOUND,
            constants::NOT_FOUND,
        )),
    }
}
