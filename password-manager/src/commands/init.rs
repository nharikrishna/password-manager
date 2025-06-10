use std::io::{self, Write};
use std::fs;
use crate::crypto;

pub fn init_vault(meta_path: &str) -> io::Result<()> {
    print!("Create Master Password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();
    
    print!("Confirm Master Password: ");
    io::stdout().flush()?;
    let mut confirm_password = String::new();
    io::stdin().read_line(&mut confirm_password)?;
    let confirm_password = confirm_password.trim();
    
    if password != confirm_password {
        println!("Passwords don't match");
        return Ok(());
    }
    
    let hashed_password = crypto::hash_password(&password)?;
    fs::write(meta_path, hashed_password)?;
    println!("Vault Initialized.");
    
    Ok(())
}