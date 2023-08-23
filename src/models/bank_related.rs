use chrono::{NaiveDateTime, Utc, DateTime};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPool;
use nanoid::nanoid;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankAccount {
    id: String,
    #[serde(skip_serializing)]
    user_id: String,
    name: String,
    description: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    include_in_total: bool,
    #[serde(skip_serializing)]
    created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    deleted_at: Option<DateTime<Utc>>,
}

impl BankAccount{


    pub fn create(user_id: String, name:String, description: Option<String>, iban: Option<String>, bic: Option<String>, include_in_total: bool)-> BankAccount{
        let bank_account_created = BankAccount{
            id: nanoid!(),
            user_id,
            name,
            description,
            iban,
            bic,
            include_in_total:include_in_total,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        bank_account_created      
    }

    pub async fn create_db(&self, pool:MySqlPool){
        let result = sqlx::query!(
            "INSERT INTO PREFIX_BankAccount (id,user_id,name,description,iban,bic,include_in_total,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?); ",
            self.id,
            self.user_id,
            self.name,
            self.description,
            self.iban,
            self.bic,
            self.include_in_total,
            self.created_at,
            self.updated_at,
        )
        .execute(&pool)
        .await
        .unwrap();
    }
    pub async fn list_all_by_user_id(user_id: String,pool:MySqlPool)->Vec<BankAccount>{
        let result = sqlx::query_as::<_,BankAccount>("SELECT * FROM PREFIX_BankAccount WHERE user_id = ? AND deleted_at IS NULL").bind(user_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        result
    }

    pub async fn get_by_id_and_user_id(bank_account_id: String,user_id: String,pool:MySqlPool)->Option<BankAccount>{
        let result = sqlx::query_as::<_,BankAccount>("SELECT * FROM PREFIX_BankAccount WHERE id = ? AND deleted_at IS NULL AND user_id = ?").bind(bank_account_id).bind(user_id)
        .fetch_one(&pool)
        .await;
        if result.is_err(){
            return None;
        }
        Some(result.unwrap())
    }
    
    pub async fn update_db(self, pool:MySqlPool, user_id: String){
        let result = sqlx::query!(
            "UPDATE PREFIX_BankAccount SET name = ?, description = ?, iban = ?, bic = ?, include_in_total = ?, updated_at = ? WHERE id = ? AND user_id = ?; ",
            self.name,
            self.description,
            self.iban,
            self.bic,
            self.include_in_total,
            Utc::now(),
            self.id,
            user_id,
        )
        .execute(&pool)
        .await
        .unwrap();
    }
    pub async fn delete_db(self, pool:MySqlPool,user_id: String){
        let result = sqlx::query!(
            "UPDATE PREFIX_BankAccount SET deleted_at = ? WHERE id = ? AND user_id = ?; ",
            Utc::now(),
            self.id,
            user_id,
        )
        .execute(&pool)
        .await
        .unwrap();
    }



}



#[derive(Debug, Serialize, Deserialize)]
struct BankAccountEntry {
    id: String,
    bank_account_id: String,
    category_id: Option<String>,
    name: String,
    description: Option<String>,
    amount: f64,
    transaction_date: DateTime<Utc>,
    #[serde(skip_serializing)]
    created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    deleted_at: Option<DateTime<Utc>>,
}



#[derive(Debug, Serialize, Deserialize)]
struct BankAccountEntryTag {
    bank_account_entry_id: String,
    tag_id: String,
    #[serde(skip_serializing)]
    created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    updated_at: DateTime<Utc>,
}