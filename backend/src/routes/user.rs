use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::*;
use uuid::Uuid;

use crate::{
    db::{
        models::{NewUser, User},
        schema::user_account,
    },
    http_utils::{self, ErrorResponse},
};

// #[debug_handler]
pub async fn get_users(
    State(db_connection_pool): State<Pool>,
) -> Result<Json<Vec<User>>, ErrorResponse> {
    let db_connection = db_connection_pool
        .get()
        .await
        .map_err(http_utils::internal_error)?;

    let result = db_connection
        .interact(|db_connection| {
            user_account::table
                .select(User::as_select())
                .load(db_connection)
        })
        .await
        .map_err(http_utils::internal_error)?
        .map_err(http_utils::internal_error)?;

    Ok(Json(result))
}

pub async fn create_user(
    State(db_connection_pool): State<Pool>,
    Json(body): Json<NewUser>,
) -> Result<StatusCode, ErrorResponse> {
    let db_connection = db_connection_pool
        .get()
        .await
        .map_err(http_utils::internal_error)?;

    db_connection
        .interact(|db_connection| {
            diesel::insert_into(user_account::table)
                .values(NewUser {
                    id: Uuid::new_v4(),
                    ..body
                })
                .execute(db_connection)
        })
        .await
        .map_err(http_utils::internal_error)?
        .map_err(http_utils::internal_error)?;

    Ok(StatusCode::CREATED)
}
