// @generated automatically by Diesel CLI.

diesel::table! {
    expense_list (id) {
        id -> Uuid,
        title -> Text,
    }
}
