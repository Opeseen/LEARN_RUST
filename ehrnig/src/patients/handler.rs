use crate::infrastructures::db;
use crate::jobs::publish_event;
use crate::patients::{CreatePatientDto, repository};
use crate::routes::AppState;
use crate::utility::{ApiError, ApiResponse, constants, extract_org_id};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn create_patient(
    State(state): State<AppState>,
    header: HeaderMap,
    Json(payload): Json<CreatePatientDto>,
) -> Result<impl IntoResponse, ApiError> {
    let org_id = extract_org_id(&header)?;

    let mut tx = state.db.begin().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    db::set_tenant_context(&mut tx, org_id).await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    let patient_id = repository::save(&mut tx, org_id, &payload)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
        })?;

    tx.commit().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;
    // Trigger the Background Event (Asynchronous)
    let mq_pool = state.mq;

    tokio::spawn(async move {
        if let Err(e) = publish_event(&mq_pool, constants::EVENT_PATIENT_CREATED, &payload).await {
            tracing::error!("MQ Error: Failed to publish: {:?}", e);
        } else {
            tracing::info!("MQ Success: Event published for {:?}", &payload);
        }
    });

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            patient_id,
            constants::CREATED,
            StatusCode::CREATED,
        )),
    ))
}

pub async fn get_patient(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    header: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let org_id = extract_org_id(&header)?;

    let mut tx = state.db.begin().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    db::set_tenant_context(&mut tx, org_id).await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    let patient = repository::find_by_id(&mut tx, id).await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    // Explicitly Commit (Clears the SET LOCAL and returns connection to pool)
    tx.commit().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    match patient {
        Some(p) => Ok(Json(ApiResponse::success(
            p,
            constants::SUCCESS,
            StatusCode::OK,
        ))),
        None => Err(ApiResponse::error(
            StatusCode::NOT_FOUND,
            constants::NOT_FOUND,
        )),
    }
}

pub async fn get_all_patient(
    State(state): State<AppState>,
    header: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let org_id = extract_org_id(&header)?;

    let mut tx = state.db.begin().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    db::set_tenant_context(&mut tx, org_id).await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    let patients = repository::find_all(&mut tx).await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    // Explicitly Commit (Clears the SET LOCAL and returns connection to pool)
    tx.commit().await.map_err(|_| {
        ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
    })?;

    Ok(Json(ApiResponse::success(
        patients,
        constants::SUCCESS,
        StatusCode::OK,
    )))
}
