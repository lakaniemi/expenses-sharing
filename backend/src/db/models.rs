use crate::db::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::expense_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExpenseList {
    pub id: Uuid,
    pub title: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::user_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::user_account)]
pub struct NewUser {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: String,
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
