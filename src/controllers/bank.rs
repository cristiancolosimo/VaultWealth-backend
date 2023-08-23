use serde::{Deserialize, Serialize};
use axum::{
    routing::{get, post},
    http::{StatusCode, Request, HeaderMap},
    Json, Router, extract::State,
    extract::{Extension,FromRequest,Path}, body::Body
};
use sqlx::mysql::MySqlPool;

use crate::{utils::jwt::verify_access_token, models::response::GenericResponse};

use crate::models::bank_related::BankAccount;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreateBankPayload{
    name: String,
    description: Option<String>,
    iban: Option<String>,
    bic: Option<String>,
    include_in_total: bool,
}


pub fn is_verified_fake_middleware(headers: HeaderMap)->Option<String>{
    let token = headers.get("Authorization");
    if token.is_none(){
        return None;
    }
    let token = token.unwrap();
    let token = token.to_str().unwrap();
    let clean_token = token.replace("Bearer ","");

    let is_verified: Option<String> = verify_access_token(clean_token);
    is_verified
}
pub async fn create_bank(State(pool): State<MySqlPool>,  headers:HeaderMap ,Json(create_bank_req): Json<RequestCreateBankPayload>)-> (StatusCode, Json<GenericResponse<BankAccount>>){
    let is_verified = is_verified_fake_middleware(headers);
    if is_verified.is_none(){
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR NOT AUTORIZED"), data: None }));
    }
    let user_id = is_verified.unwrap();
    let bank = BankAccount::create(user_id,create_bank_req.name,create_bank_req.description,create_bank_req.iban,create_bank_req.bic,create_bank_req.include_in_total);
    bank.create_db(pool).await;
    return (StatusCode::OK, Json(GenericResponse { message: String::from("OK"), data: Some(bank) }));

}
pub async fn list_all_banks(State(pool): State<MySqlPool>, headers:HeaderMap)-> (StatusCode, Json<GenericResponse<Vec<BankAccount>>>){
    let is_verified: Option<String> = is_verified_fake_middleware(headers);
    if is_verified.is_none(){
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR NOT AUTORIZED"), data: None }));
    }
    let user_id = is_verified.unwrap();
    let banks = BankAccount::list_all_by_user_id(user_id,pool).await;

    return (StatusCode::OK, Json(GenericResponse { message: String::from("OK"), data: Some(banks) }));

}
pub async fn get_bank(State(pool): State<MySqlPool>, headers:HeaderMap, Path(bank_id): Path<String>)->(StatusCode, Json<GenericResponse<BankAccount>>){
    let is_verified: Option<String> = is_verified_fake_middleware(headers);
    if is_verified.is_none(){
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR NOT AUTORIZED"), data: None }));
    }
    let user_id = is_verified.unwrap();
    let bank = BankAccount::get_by_id_and_user_id(bank_id,user_id,pool).await;
    println!("{:?}",bank);
    return (StatusCode::OK, Json(GenericResponse { message: String::from("OK"), data: bank }));

}
pub async fn update_bank(State(pool): State<MySqlPool>,Path(bank_id): Path<String>, headers:HeaderMap,Json(create_bank_req): Json<RequestCreateBankPayload>,)-> (StatusCode, Json<GenericResponse<BankAccount>>){
    let is_verified = is_verified_fake_middleware(headers);
    if is_verified.is_none(){
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR NOT AUTORIZED"), data: None }));
    }
    let user_id = is_verified.unwrap();
    let bank = BankAccount::get_by_id_and_user_id(bank_id,user_id.clone(),pool.clone()).await;
    if bank.is_none(){
        return (StatusCode::NOT_FOUND, Json(GenericResponse { message: String::from("ERROR NOT FOUND"), data: None }));
    }
    let mut bank = bank.unwrap();
    bank.name = create_bank_req.name;
    bank.description = create_bank_req.description;
    bank.iban = create_bank_req.iban;
    bank.bic = create_bank_req.bic;
    bank.include_in_total = create_bank_req.include_in_total;
    bank.update_db(pool,user_id).await;

    return (StatusCode::OK, Json(GenericResponse { message: String::from("OK"), data: Some(bank) }));
}
pub async fn delete_bank(State(pool): State<MySqlPool>,Path(bank_id): Path<String>, headers:HeaderMap)-> (StatusCode, Json<GenericResponse<String>>){
    let is_verified = is_verified_fake_middleware(headers);
    if is_verified.is_none(){
        return (StatusCode::UNAUTHORIZED, Json(GenericResponse { message: String::from("ERROR NOT AUTORIZED"), data: None }));
    }
    let user_id = is_verified.unwrap();
    let bank = BankAccount::get_by_id_and_user_id(bank_id,user_id.clone(),pool.clone()).await;
    if bank.is_none(){
        return (StatusCode::NOT_FOUND, Json(GenericResponse { message: String::from("ERROR NOT FOUND"), data: None }));
    }
    let bank = bank.unwrap();
    bank.delete_db(pool,user_id).await;
    return (StatusCode::OK, Json(GenericResponse { message: String::from("OK"), data: Some(String::from("DELETED")) }));

}

