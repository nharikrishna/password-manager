use argon2::{Argon2, PasswordHasher};
use password_hash::{PasswordHash, PasswordVerifier, SaltString};
use rand::rngs::OsRng;
use std::io;

pub fn hash_password(password: &str) -> io::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let hash = argon2
    .hash_password(password.as_bytes(), &salt)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
    .to_string();
    
    Ok(hash)
}

pub fn verify_password(password: &str, hashed: &str) -> io::Result<bool> {
    let parsed_hash = PasswordHash::new(hashed)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password)  => Ok(false),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}