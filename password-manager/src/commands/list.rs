use std::io;
use crate::storage::load_vault_encrypted;

pub fn list_services(vault_path: &str, key: &[u8; 32]) -> io::Result<()> {
    let vault = load_vault_encrypted(&vault_path, &key).unwrap_or_default();
    if vault.credentials.is_empty() {
        println!("\nNo credentials stored.");
    } else {
        println!("\nStored services:");
        for service in vault.credentials.keys() {
            println!("{}", service);
        }
    }
    
    Ok(())
}