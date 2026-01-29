INSERT INTO bank (id, bank_name) VALUES
    (1, 'Bank of America'),
    (2, 'Wells Fargo');

INSERT INTO account (id, name, bank_id, account_type, initial_balance_cents, current_balance_cents) VALUES
    (1, 'Primary Checking', 1, 'CHECKINGS', 100000, 125050),
    (2, 'Emergency Fund', 1, 'SAVINGS', 500000, 520000),
    (3, 'Joint Checking', 2, 'CHECKINGS', 250000, 248075);
