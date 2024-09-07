use self::models::{ExpenseList, NewExpenseList};
use self::schema::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use expenses_sharing_backend::*;
use uuid::Uuid;

fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let connection = &mut establish_connection();

    let new_expense_list = NewExpenseList {
        title: "Asd".to_string(),
        id: Uuid::new_v4(),
    };

    diesel::insert_into(expense_list::table)
        .values(&new_expense_list)
        .returning(ExpenseList::as_returning())
        .get_result(connection)
        .expect("Error saving new expense list");

    let results = expense_list::table
        .limit(5)
        .select(ExpenseList::as_select())
        .load(connection)
        .expect("Error loading expense lists");

    println!("Displaying {} expense lists", results.len());
    for expense_list in results {
        println!("{}", expense_list.title);
    }
}
