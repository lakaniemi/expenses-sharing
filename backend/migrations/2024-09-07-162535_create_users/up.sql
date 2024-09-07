-- Your SQL goes here

CREATE TABLE user_account (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE user_account_expense_list (
  id SERIAL PRIMARY KEY,
  user_account_id UUID NOT NULL,
  expense_list_id UUID NOT NULL,

  CONSTRAINT fk_user_account
    FOREIGN KEY(user_account_id) 
	    REFERENCES user_account(id)
	    ON DELETE CASCADE,
  CONSTRAINT fk_expense_list
    FOREIGN KEY(expense_list_id) 
	    REFERENCES expense_list(id)
	    ON DELETE CASCADE
)