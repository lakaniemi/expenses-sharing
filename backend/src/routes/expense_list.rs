use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::*;

use crate::{
    db::{models::ExpenseList, schema::expense_list},
    http_errors,
};

// #[debug_handler]
pub async fn get_expense_lists(
    State(db_connection_pool): State<Pool>,
) -> Result<Json<Vec<ExpenseList>>, (StatusCode, String)> {
    let db_connection = db_connection_pool
        .get()
        .await
        .map_err(http_errors::internal_error)?;

    let result: Vec<ExpenseList> = db_connection
        .interact(|db_connection| {
            expense_list::table
                .select(ExpenseList::as_select())
                .load(db_connection)
        })
        .await
        .map_err(http_errors::internal_error)?
        .map_err(http_errors::internal_error)?;

    Ok(Json(result))
}
