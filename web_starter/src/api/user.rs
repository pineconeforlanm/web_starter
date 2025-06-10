use axum::{debug_handler, Router};
use axum::extract::State;
use sea_orm::prelude::*;
use crate::app::AppState;
use crate::entity::prelude::SysUser;
use crate::entity::sys_user;
use crate::error::ApiResult;
use crate::response::ApiResponse;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(query_users))
}

#[debug_handler]
async fn query_users(State(AppState { db }): State<AppState>) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        .filter(sys_user::Column::Gender.eq("male"))
        .all(&db)
        .await
        .expect("Failed to query users");
    Ok(ApiResponse::ok("ok", Some(users)))
}