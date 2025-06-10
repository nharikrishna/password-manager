use crate::models::Vault;
use rand::RngCore;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use base64::Engine;
use base64::engine::general_purpose;
use rand::rngs::OsRng;
use serde_json::{json, Value};


pub fn save_vault_encrypted(path: &str, vault: &Vault, key: &[u8; 32], salt: &Vec<u8>) -> io::Result<()> {
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    
    let data = serde_json::to_string(vault)?;
    let data_bytes = data.as_bytes();
    
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let cipher_text = cipher.encrypt(nonce, data_bytes).unwrap();

    let mut file = File::create(path)?;
    
    let file_data = json!({
        "salt": general_purpose::STANDARD.encode(&salt),
        "nonce": general_purpose::STANDARD.encode(&nonce_bytes),
        "ciphertext": general_purpose::STANDARD.encode(&cipher_text),
    });
    
    file.write_all(file_data.to_string().as_bytes())?;

    Ok(())
}

pub fn load_vault_encrypted(path: &str, key: &[u8; 32]) -> io::Result<Vault> {
    let file_data = fs::read_to_string(path)?;
    let json_data: Value = serde_json::from_str(&file_data)?;
    
    let salt_b64 = json_data["salt"].as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "salt is missing"))?;
    let nonce_b64 = json_data["nonce"].as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "nonce is missing"))?;
    let ciphertext_b64 = json_data["ciphertext"].as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "ciphertext is missing"))?;
    
    let salt = general_purpose::STANDARD.decode(salt_b64)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let nonce = general_purpose::STANDARD.decode(nonce_b64)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    if salt.len() != 16 || nonce.len() != 12 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid salt or Nonce length"));
    }

    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = Nonce::from_slice(nonce.as_slice());
    
    let decrypted_bytes = cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decryption Failed"))?;
    
    let vault: Vault = serde_json::from_slice(&decrypted_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    Ok(vault)
}