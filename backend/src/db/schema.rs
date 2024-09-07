// @generated automatically by Diesel CLI.

diesel::table! {
    expense (id) {
        id -> Uuid,
        expense_list_id -> Uuid,
        title -> Text,
        amount_cents -> Int8,
        #[max_length = 3]
        currency_code -> Bpchar,
        paid -> Bool,
    }
}

diesel::table! {
    expense_list (id) {
        id -> Uuid,
        title -> Text,
    }
}

diesel::table! {
    user_account (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    user_account_expense_list (id) {
        id -> Int4,
        user_account_id -> Uuid,
        expense_list_id -> Uuid,
    }
}

diesel::joinable!(expense -> expense_list (expense_list_id));
diesel::joinable!(user_account_expense_list -> expense_list (expense_list_id));
diesel::joinable!(user_account_expense_list -> user_account (user_account_id));

diesel::allow_tables_to_appear_in_same_query!(
    expense,
    expense_list,
    user_account,
    user_account_expense_list,
);
