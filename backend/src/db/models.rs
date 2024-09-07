use crate::db::schema;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::expense_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExpenseList {
    pub id: Uuid,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::expense_list)]
// pub struct NewExpenseList<'a> {
//     pub title: &'a str,
// }
pub struct NewExpenseList {
    pub id: Uuid,
    pub title: String,
}
