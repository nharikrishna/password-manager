use std::io;
use crate::storage::{load_vault_encrypted, save_vault_encrypted};

pub fn delete_credential(vault_path: &str, key: &[u8; 32], salt: &Vec<u8>, 
                         service_name: &str) -> io::Result<()> {
    
    let mut vault = load_vault_encrypted(vault_path, &key)?;
    if vault.credentials.remove(service_name).is_some() {
        save_vault_encrypted(vault_path, &vault, &key, &salt)?;
        println!("\nCredential removed for service: {}", service_name);
    } else {
        println!("\nCredential not found for service: {}", service_name);
    }
    
    Ok(())
}