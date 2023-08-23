use serde::{Serialize, Deserialize};
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use jsonwebtoken::encode;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;

use crate::models::user::User;



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    //aud: String,         // Optional. Audience
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64, // Optional. Issued at (as UTC timestamp) //quanto è stato creato
    iss: String, // Optional. Issuer //creatore
    //nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
    //role: i32,
}


pub fn generate_access_token(user_id: String)-> String{
    let key_access = std::env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET");

    let time_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let scadenza = std::time::Duration::new(60 * 60, 0);

    let exp = time_now + scadenza;
    let exp = exp.as_secs();
    let time_now = time_now.as_secs();

    let access_token = Claims {
        sub: user_id,
        exp,
        iat: time_now,
        iss: String::from("super cool server"),
        
    };
    let access_token_encoded = match encode(
        &Header::default(),
        &access_token,
        &EncodingKey::from_secret(key_access.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    println!("token_access: {}", access_token_encoded);
    return access_token_encoded;
}

pub fn verify_access_token(token:String)->Option<String>{
    let key_access = std::env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET");
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key_access.as_bytes()),
        &Validation::default(),
    ) {
        Ok(c) => {
            return Some(c.claims.sub);
        },
        Err(_) => {
            return None;
        }
    };
    
    
}