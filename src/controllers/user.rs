use serde::{Deserialize, Serialize};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::State,
};
use sqlx::mysql::MySqlPool;

use crate::models::user::{User, UserRegistrationLogin};



pub async fn registration(State(pool): State<MySqlPool>,Json(payload): Json<UserRegistrationLogin>) -> (StatusCode, Json<UserRegistrationLogin>) {
    
    let user: User = User::create(payload.email.clone(),payload.password.clone());
    user.create_db(pool).await;
    
    (StatusCode::CREATED, Json(payload))
}

pub async fn login(State(pool): State<MySqlPool>,Json(payload): Json<UserRegistrationLogin>)-> (StatusCode, Json<UserRegistrationLogin>) {
    let user = User::login_db(pool,payload.email.clone(),payload.password.clone()).await;
    if user.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(payload));
    }
    
    (StatusCode::ACCEPTED, Json(payload))
}