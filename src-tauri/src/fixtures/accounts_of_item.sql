INSERT INTO bank (id, plaid_item_id, bank_name) VALUES
    (1, 'item-1', 'Bank of America'),
    (2, 'item-2', 'Wells Fargo');

-- Accounts 1 and 2 belong to item-1, account 3 to item-2, and account 4 is a
-- manual account with no linked item.
INSERT INTO account (id, plaid_account_id, name, bank_id, plaid_item_id, account_type) VALUES
    (1, 'plaid-acct-1', 'Primary Checking', 1, 'item-1', 'CHECKINGS'),
    (2, 'plaid-acct-2', 'Emergency Fund', 1, 'item-1', 'SAVINGS'),
    (3, 'plaid-acct-3', 'Joint Checking', 2, 'item-2', 'CHECKINGS'),
    (4, NULL, 'Manual Savings', 1, NULL, 'SAVINGS');
