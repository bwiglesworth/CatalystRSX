use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};
use base64::engine::general_purpose::STANDARD;
use std::env;
use base64::Engine;

pub fn verify_password(password: &str, phc_string: &str) -> bool {
    println!("Attempting to verify password: {}", password);

    
    let parsed_hash = PasswordHash::new(phc_string).unwrap();
    println!("Parsed hash: {:?}", parsed_hash);
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn generate_hash(password: &str) -> String {
    let salt = env::var("ARGON2_SALT").expect("ARGON2_SALT must be set");
    let params = argon2::Params::new(19456, 2, 1, None).unwrap();
    
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params
    );
    
    let mut output = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output).unwrap();
    format!("$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ${}", STANDARD.encode(&output))
}
