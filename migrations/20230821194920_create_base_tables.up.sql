-- Add up migration script here
CREATE TABLE PREFIX_User (
    id VARCHAR(25) PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(64) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);


CREATE TABLE PREFIX_BankAccount (
    id VARCHAR(25) PRIMARY KEY,
    user_id VARCHAR(25) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    iban VARCHAR(255),
    bic VARCHAR(255),
    include_in_total BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
    /*,
    FOREIGN KEY (user_id) REFERENCES PREFIX_User(id)*/
);

CREATE TABLE PREFIX_BankAccountEntry (
    id VARCHAR(25) PRIMARY KEY,
    bank_account_id VARCHAR(25) NOT NULL,
    category_id VARCHAR(25),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    amount DECIMAL(10, 2) NOT NULL,
    transaction_date DATE NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
    /*,
    FOREIGN KEY (bank_account_id) REFERENCES PREFIX_BankAccount(id),
    FOREIGN KEY (category_id) REFERENCES PREFIX_Category(id)*/
);


CREATE TABLE PREFIX_Category (
    id VARCHAR(25) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE PREFIX_Tag (
    id VARCHAR(25) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL
);

CREATE TABLE PREFIX_BankAccountEntryTag (
    bank_account_entry_id VARCHAR(25) NOT NULL,
    tag_id VARCHAR(25) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    PRIMARY KEY (bank_account_entry_id, tag_id),
    deleted_at TIMESTAMP DEFAULT NULL/*,
    FOREIGN KEY (bank_account_entry_id) REFERENCES PREFIX_BankAccountEntry(id),
    FOREIGN KEY (tag_id) REFERENCES PREFIX_Tag(id)*/
);

