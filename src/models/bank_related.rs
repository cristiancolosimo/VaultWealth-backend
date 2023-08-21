use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct BankAccount {
    id: String,
    user_id: String,
    name: String,
    description: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    include_in_total: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,

}

#[derive(Debug, Serialize, Deserialize)]
struct BankAccountEntry {
    id: String,
    bank_account_id: String,
    category_id: Option<String>,
    name: String,
    description: Option<String>,
    amount: f64,
    transaction_date: NaiveDate,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize)]
struct BankAccountEntryTag {
    bank_account_entry_id: String,
    tag_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}