use crate::error::ApiError;
use axum::extract::FromRequestParts;
use axum_valid::HasValidate;

#[derive(Debug, Clone, Default, FromRequestParts)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl<T> HasValidate for Json<T> {
    type Validate = T;
    fn get_validate(&self) -> &Self::Validate {
        &self.0
    }
}
