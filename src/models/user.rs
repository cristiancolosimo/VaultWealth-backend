use chrono::{NaiveDateTime, Utc, DateTime};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;
use sqlx::{mysql::MySqlPool};

use crate::utils::password::verify_password;
#[derive(Debug, Serialize, Deserialize,sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn set_password(&mut self,password: String) {
        self.password = password;
    }
    pub fn verify_password(self,password: String) -> bool {
        self.password == password
    }

    pub fn get(self)-> User {
        let tmp = User{
            id: self.id,
            email: self.email,
            password: "********".to_string(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        };
        tmp      
    }
    pub fn create(email: String, password:String)-> User{
        let password_hashed = crate::utils::password::hash_password(password.clone());
        let user_created = User{
            id: nanoid!(),
            email: email,
            password: password_hashed,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };
        user_created      
    }

    pub async fn create_db(self,pool:MySqlPool){
        let result = sqlx::query!(
            "INSERT INTO PREFIX_User (id,email,password,created_at,updated_at) VALUES (?,?,?,?,?); ",
            self.id,
            self.email,
            self.password,
            self.created_at,
            self.updated_at,
        )
        .execute(&pool)
        .await
        .unwrap();
        println!("result: {:?}", result);
    }

    pub async fn login_db(pool: MySqlPool, email:String, password:String)->Option<User>{
        let result = sqlx::query!("SELECT * FROM PREFIX_User WHERE email = ?;",email)
        .fetch_one(&pool)
        .await;
        println!("result: {:?}", result);
        
        if result.is_err(){
            return None;
        }
        let user = result.unwrap();
        let user = User{
            id: user.id,
            email: user.email,
            password: user.password,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        };
    
        let is_verified = verify_password(password,user.password.clone());
        if !is_verified{
            return None;
        }
        Some(user)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationLogin {
    pub email: String,
    pub password: String,
}