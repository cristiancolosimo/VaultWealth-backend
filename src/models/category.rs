use chrono::{NaiveDateTime, Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    id: String,
    #[serde(skip_serializing)]
    user_id: String,
    name: String,
    description: Option<String>,
    #[serde(skip_serializing)]
    created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    updated_at: NaiveDateTime,
    #[serde(skip_serializing)]
    deleted_at: Option<NaiveDateTime>,

}
