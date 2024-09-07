use axum::{extract::State, Json};
use deadpool_diesel::postgres::Pool;
use diesel::*;

use crate::{
    db::{models::ExpenseList, schema::expense_list},
    http_utils::{self, ErrorResponse},
};

// #[debug_handler]
pub async fn get_expense_lists(
    State(db_connection_pool): State<Pool>,
) -> Result<Json<Vec<ExpenseList>>, ErrorResponse> {
    let db_connection = db_connection_pool
        .get()
        .await
        .map_err(http_utils::internal_error)?;

    let result = db_connection
        .interact(|db_connection| {
            expense_list::table
                .select(ExpenseList::as_select())
                .load(db_connection)
        })
        .await
        .map_err(http_utils::internal_error)?
        .map_err(http_utils::internal_error)?;

    Ok(Json(result))
}
