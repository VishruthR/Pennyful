CREATE TABLE IF NOT EXISTS bank (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plaid_item_id TEXT,
    plaid_institution_id TEXT,
    bank_name TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_plaid_item_id
  ON bank(plaid_item_id)
  WHERE plaid_item_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plaid_account_id TEXT,
    name TEXT NOT NULL,
    official_name TEXT,
    bank_id INTEGER NOT NULL,
    plaid_item_id TEXT,
    account_type TEXT NOT NULL CHECK(account_type IN ('SAVINGS', 'CHECKINGS', 'CREDIT')),
    initial_balance_cents INTEGER NOT NULL DEFAULT 0,
    available_balance_cents INTEGER NOT NULL DEFAULT 0,
    current_balance_cents INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (bank_id) REFERENCES bank(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_account_bank_id ON account(bank_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_plaid_account_id
  ON account(plaid_account_id)
  WHERE plaid_account_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS category (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    color TEXT NOT NULL CHECK(length(color) <= 7),
    icon TEXT
);

CREATE TABLE IF NOT EXISTS "transaction" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plaid_transaction_id STRING,
    name TEXT NOT NULL,
    merchant_entity_id TEXT,
    amount_cents INTEGER NOT NULL,
    date TEXT NOT NULL,
    pending INTEGER NOT NULL DEFAULT 0 CHECK (pending IN (0, 1)),
    deleted_at TEXT,
    plaid_account_id TEXT,
    account_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE SET NULL
);
CREATE INDEX IF NOT EXISTS idx_transaction_date ON "transaction"(date);
CREATE INDEX IF NOT EXISTS idx_transaction_account_id ON "transaction"(account_id);
CREATE INDEX IF NOT EXISTS idx_transaction_category_id ON "transaction"(category_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_transaction_plaid_transaction_id
    ON "transaction"(plaid_transaction_id)
    WHERE plaid_transaction_id IS NOT NULL;
