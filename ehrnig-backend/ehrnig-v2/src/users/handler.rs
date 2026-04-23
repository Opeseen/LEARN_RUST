use crate::auth::hash_password;
use crate::routes::AppState;
use crate::users::{RegisterUserDto, RepoError, repository};
use crate::utility::{ApiError, ApiResponse, constants};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn create_user(
    State(state): State<AppState>,
    Json(mut payload): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, ApiError> {
    // hash the user password first
    payload.password = hash_password(&payload.password)
        .await
        .map_err(|e| ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // create the user
    let user_id = repository::create(&state.db, &payload)
        .await
        .map_err(|e| match e {
            RepoError::AlreadyExists(_) => ApiResponse::error(StatusCode::CONFLICT, &e.to_string()),
            RepoError::Database(err) => {
                tracing::error!("{}", err.to_string());
                ApiResponse::error(StatusCode::INTERNAL_SERVER_ERROR, constants::INTERNAL_ERROR)
            }
        })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            user_id,
            constants::CREATED,
            StatusCode::CREATED,
        )),
    ))
}
