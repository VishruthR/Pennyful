CREATE TABLE IF NOT EXISTS bank (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bank_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bank_id INTEGER NOT NULL,
    account_type TEXT NOT NULL CHECK(account_type IN ('SAVINGS', 'CHECKINGS')),
    initial_balance REAL NOT NULL DEFAULT 0.0,
    current_balance REAL NOT NULL DEFAULT 0.0,
    FOREIGN KEY (bank_id) REFERENCES bank(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_account_bank_id ON account(bank_id);

CREATE TABLE IF NOT EXISTS category (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL CHECK(length(color) <= 7)
);

CREATE TABLE IF NOT EXISTS "transaction" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    amount REAL NOT NULL,
    date TEXT NOT NULL,
    account_id INTEGER NOT NULL,
    category_id INTEGER,
    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS idx_transaction_date ON "transaction"(date);
CREATE INDEX IF NOT EXISTS idx_transaction_account_id ON "transaction"(account_id);
CREATE INDEX IF NOT EXISTS idx_transaction_category_id ON "transaction"(category_id);
