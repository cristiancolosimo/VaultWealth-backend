use argon2::{self, Argon2, PasswordHash, password_hash::{SaltString, rand_core::OsRng}, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: String) -> String {
    
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    password_hash
}


pub fn verify_password(password: String, password_hash: String)->bool{

    let parsed_hash = PasswordHash::new(&password_hash);
    if parsed_hash.is_err(){
        return false;
    }
    let parsed_hash = parsed_hash.unwrap();
    let is_valid = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    if let Err(e) = is_valid {
        println!("Error: {}", e);
        return false;
    }
    return true;
}