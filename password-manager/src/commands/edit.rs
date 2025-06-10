use std::collections::HashMap;
use std::io;
use crate::models::Vault;
use crate::storage::{load_vault_encrypted, save_vault_encrypted};

pub fn edit_credential(
    vault_path: &str, key: &[u8; 32], salt: &Vec<u8>, service_name: String, 
    new_username: String, new_password: String) -> io::Result<()> {
    let mut vault = load_vault_encrypted(vault_path, key)
        .unwrap_or_else(|_| Vault { credentials: HashMap::new() });

    if let Some(credential) = vault.credentials.get_mut(&service_name) {
        credential.username = new_username;
        credential.password = new_password;

        save_vault_encrypted(vault_path, &vault, key, salt)?;
        println!("\nCredential updated for '{}'", service_name);
    } else {
        println!("\nNo credential found for '{}'", service_name);
    }

    Ok(())
}
