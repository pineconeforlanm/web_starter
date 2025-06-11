use crate::app::AppState;
use crate::common::{Page, PaginationParams};
use crate::entity::prelude::SysUser;
use crate::entity::sys_user;
use crate::error::ApiResult;
use crate::response::ApiResponse;
use crate::valid::ValidQuery;
use axum::extract::State;
use axum::{Router, debug_handler};
use sea_orm::prelude::*;
use sea_orm::{Condition, QueryOrder, QueryTrait};
use serde::Deserialize;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", axum::routing::get(find_page))
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    keyword: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[tracing::instrument(name = "query_users")]
#[debug_handler]
async fn find_page(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
) -> ApiResult<ApiResponse<Page<sys_user::Model>>> {
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(&db, pagination.size);

    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page - 1).await?;

    let page = Page::from_pagination_params(pagination, total, items);

    Ok(ApiResponse::ok("ok", Some(page)))
}
