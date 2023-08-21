-- Add down migration script here

-- Drop BankAccountEntryTag Table
DROP TABLE IF EXISTS PREFIX_BankAccountEntryTag;

-- Drop BankAccountEntry Table
DROP TABLE IF EXISTS PREFIX_BankAccountEntry;

-- Drop Category Table
DROP TABLE IF EXISTS PREFIX_Category;

-- Drop Tag Table
DROP TABLE IF EXISTS PREFIX_Tag;

-- Drop BankAccount Table
DROP TABLE IF EXISTS PREFIX_BankAccount;

-- Drop User Table
DROP TABLE IF EXISTS PREFIX_User;
