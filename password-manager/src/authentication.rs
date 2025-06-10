use std::{fs, io};
use std::io::{ErrorKind, Write};
use argon2::Argon2;
use base64::Engine;
use base64::engine::general_purpose;
use rand::RngCore;
use rand::rngs::OsRng;
use rpassword::read_password;
use serde_json::Value;
use crate::crypto::verify_password;

pub fn authenticate(meta_path: &str, vault_path: &str) -> io::Result<([u8; 32], Vec<u8>)> {
    print!("Enter Master Password: ");
    io::stdout().flush()?;
    let master_password = read_password()?;

    let stored_hash = fs::read_to_string(meta_path)?.trim().to_string();

    let password_matches = verify_password(&master_password, &stored_hash)?;
    if !password_matches {
        eprintln!("\nAuthentication failed");
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Master password doesn't match"));
    }
    
    derive_key_from_password(vault_path, &master_password)
}

fn derive_key_from_password(path: &str, password: &str) -> io::Result<([u8; 32], Vec<u8>)> {
    let salt = match fs::read_to_string(path) {
        Ok(file_data) => {
            let json_data: Value = serde_json::from_str(&file_data)
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            let salt_b64 = json_data["salt"].as_str();
            match salt_b64 {
                Some(s) => general_purpose::STANDARD.decode(s)
                    .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?,
                None => {
                    let mut salt = vec![0u8; 16];
                    OsRng.fill_bytes(&mut salt);
                    salt
                }
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let mut salt = vec![0u8; 16];
            OsRng.fill_bytes(&mut salt);
            salt
        }
        Err(e) => return Err(e),
    };

    let argon2 = Argon2::default();
    let mut key = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), &salt, &mut key)
        .map_err(|_| io::Error::new(ErrorKind::Other, "Encryption key derivation failed"))?;

    Ok((key, salt))
}