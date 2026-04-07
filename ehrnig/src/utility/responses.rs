use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

// Error alias
pub type ApiError = (StatusCode, Json<ApiResponse<()>>);

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: &str, status: StatusCode) -> Self {
        Self {
            success: true,
            code: status.as_u16(),
            message: message.to_string(),
            data: Some(data),
        }
    }
}

// Error Builder (Uses <()> because we aren't returning any data)
impl ApiResponse<()> {
    pub fn error(status: StatusCode, message: &str) -> ApiError {
        let body = Self {
            success: false,
            code: status.as_u16(),
            message: message.to_string(),
            data: None,
        };
        (status, Json(body))
    }
}
