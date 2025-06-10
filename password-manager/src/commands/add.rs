use std::collections::HashMap;
use std::io;
use crate::models::{Credential, Vault};
use crate::storage::{load_vault_encrypted, save_vault_encrypted};

pub fn add_credentials(vault_path: &str, key: &[u8; 32], salt: &Vec<u8>, service: String, 
                       username: String, password: String) -> io::Result<()> {
    
    let credential = Credential {
        service: service.clone(),
        username,
        password: password.clone(),
    };
    
    let mut vault = load_vault_encrypted(vault_path, key)
        .unwrap_or_else(|_| Vault { credentials: HashMap::new() });
    
    vault.credentials.insert(service, credential);
    
    save_vault_encrypted(vault_path, &vault, key, salt)?;
    
    println!("\nCredential added");
    
    Ok(())
}