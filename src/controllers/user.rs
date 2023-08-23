use serde::{Deserialize, Serialize};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, extract::State,
};
use sqlx::mysql::MySqlPool;

use crate::{models::{user::{User, UserRegistrationLogin}, response::GenericResponse}, utils::jwt::generate_access_token};



pub async fn registration(State(pool): State<MySqlPool>,Json(payload): Json<UserRegistrationLogin>) -> (StatusCode, Json<UserRegistrationLogin>) {
    
    let user: User = User::create(payload.email.clone(),payload.password.clone());
    user.create_db(pool).await;
    
    (StatusCode::CREATED, Json(payload))
}


pub async fn login(State(pool): State<MySqlPool>,Json(payload): Json<UserRegistrationLogin>)-> (StatusCode, Json<GenericResponse<String>>) {
    let user = User::login_db(pool,payload.email.clone(),payload.password.clone()).await;
    if user.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR"), data: None }));
    }
    let token = generate_access_token(user.unwrap().id.clone());
    let response = GenericResponse{
        message: String::from("OK"),
        data: Some(token),
    };
    (StatusCode::ACCEPTED, Json(response))
}