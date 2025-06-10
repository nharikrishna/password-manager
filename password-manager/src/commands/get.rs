use std::io;
use crate::storage::load_vault_encrypted;

pub fn get_credential(vault_path: &str, key: &[u8; 32], service_name: &str) -> io::Result<()> {
    
    let vault = load_vault_encrypted(vault_path, &key);
    match vault?.credentials.get(service_name) {
        Some(credential) => {
            println!("\nFound credential for {}:", credential.service);
            println!("Username: {}", credential.username);
            println!("Password: {}", credential.password);
        }
        None => {
            println!("\nNo credential found");
        }
    }
    
    Ok(())
}