INSERT INTO bank (id, bank_name) VALUES
    (1, "Bank of America");

INSERT INTO account (id, bank_id, account_type, initial_balance_cents, current_balance_cents) VALUES
    (1, 1, "CHECKINGS", 100, 150);

INSERT INTO "transaction" (id, date, amount_cents, name, account_id, category_id) VALUES 
    (1, '2025-12-15', -577, 'TRANSACTION 1', 1, 1),
    (2, '2025-12-16', -1090, 'TRANSACTION 2', 1, 1),
    (3, '2025-12-17', -190, 'TRANSACTION 3', 1, 1),
    (4, '2025-12-16', -70, 'TRANSACTION 4', 1, 1);