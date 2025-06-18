use crate::response::ApiResponse;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::response::{IntoResponse, Response};
use axum_valid::ValidRejection;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    #[error("Method Not Allowed")]
    MethodNotAllowed,

    #[error("Database exception: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Query error: {0}")]
    Query(#[from] QueryRejection),

    #[error("Path error: {0}")]
    Path(#[from] PathRejection),

    #[error("Body error: {0}")]
    Json(#[from] JsonRejection),

    #[error("param valid failed: {0}")]
    Validation(String),

    #[error("{0}")]
    Biz(String),

    #[error("Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<ValidRejection<ApiError>> for ApiError {
    fn from(err: ValidRejection<ApiError>) -> Self {
        match err {
            ValidRejection::Valid(err) => ApiError::Validation(err.to_string()),
            ValidRejection::Inner(err) => err,
        }
    }
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Database(_) | ApiError::Internal(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::Query(_)
            | ApiError::Path(_)
            | ApiError::Json(_)
            | ApiError::Validation(_) => axum::http::StatusCode::BAD_REQUEST,
            ApiError::Biz(_) => axum::http::StatusCode::OK,
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
