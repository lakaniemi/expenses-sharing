-- Your SQL goes here

CREATE TABLE expense (
  id UUID PRIMARY KEY,
  expense_list_id UUID NOT NULL,
  title TEXT NOT NULL,
  amount_cents BIGINT NOT NULL,
  currency_code CHAR(3) NOT NULL,
  paid BOOLEAN NOT NULL DEFAULT false,

  CONSTRAINT fk_expense_list
    FOREIGN KEY(expense_list_id) 
	    REFERENCES expense_list(id)
	      ON DELETE CASCADE
)