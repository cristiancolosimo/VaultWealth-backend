# Todolist

- Login
- Registration
- Logout

- Profile

- Create Bank Account
- Update Bank Account
- Delete Bank Account
- List all Bank Accounts

- Create entry in Bank Account
- Delete entry in Bank Account
- Update entry in Bank Account
- List all entries in Bank Account 

- Create Category
- Update Category
- Delete Category
- List all Categories

- Create tag
- Update tag
- Delete tag
- List all tags


--------------------
Models 

- User
- Bank Account
- Bank Account Entry
- Category
- Tag

User 1-N Bank Account
Bank Account 1-N Bank Account Entry
Bank Account Entry N-1 Category
Bank Account Entry N-N Tag

----------------------
User (
    id NANOID(25),
    email VARCHAR(255),
    password HASH varchar(64),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)



Bank Account (
    id NANOID(25),
    user_id NANOID(25),
    name VARCHAR(255),
    description TEXT NULL,
    iban VARCHAR(255) NULL,
    bic VARCHAR(255) NULL,
    include_in_total BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)

Bank Account Entry (
    id NANOID(25),
    bank_account_id NANOID(25),
    category_id NANOID(25),
    name VARCHAR(255),
    description TEXT NULL,
    amount DECIMAL(10,2),
    transaction_date DATE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)

Category (
    id NANOID(25),
    name VARCHAR(255),
    description TEXT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)

Tag (
    id NANOID(25),
    name VARCHAR(255),
    description TEXT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)

Bank Account Entry N-N Tag (
    bank_account_entry_id NANOID(25),
    tag_id NANOID(25),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
)
