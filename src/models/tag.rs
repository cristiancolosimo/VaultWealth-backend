use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    id: String,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,

}

