INSERT INTO bank (id, bank_name) VALUES (1, 'Test Bank');

-- Account 1 is Plaid-linked; account 2 is a manual account (no plaid_account_id).
INSERT INTO account (id, plaid_account_id, name, bank_id, account_type) VALUES
    (1, 'plaid-acct-1', 'Linked Checking', 1, 'CHECKINGS'),
    (2, NULL, 'Manual Savings', 1, 'SAVINGS');
