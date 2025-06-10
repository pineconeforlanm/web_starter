use axum::response::{IntoResponse, Response};
use crate::response::ApiResponse;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    
    #[error("{0}")]
    Biz(String),
    
    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => axum::http::StatusCode::OK,
            ApiError::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let sc = self.status_code();
        
        let resp = axum::Json(ApiResponse::<()>::error(self.to_string()));

        (sc, resp).into_response()
    }
}