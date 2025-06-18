use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok<U: AsRef<str>>(message: U, data: Option<T>) -> Self {
        Self::new(0, String::from(message.as_ref()), data)
    }

    pub fn error<U: AsRef<str>>(message: U) -> Self {
        Self::new(-1, String::from(message.as_ref()), None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
