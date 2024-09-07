use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::expense_list)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExpenseList {
    pub id: Uuid,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::expense_list)]
// pub struct NewExpenseList<'a> {
//     pub title: &'a str,
// }
pub struct NewExpenseList {
    pub id: Uuid,
    pub title: String,
}
