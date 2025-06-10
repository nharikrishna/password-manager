use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::authentication;
use crate::commands;

pub struct PasswordManagerApp {
    vault_path: String,
    meta_path: String,
    key: [u8; 32],
    salt: Vec<u8>,
}

impl PasswordManagerApp {
    pub fn new() -> Self {
        let base_dir = dirs::home_dir().unwrap().join(".password-manager");

        fs::create_dir_all(&base_dir).expect("Failed to create config directory");

        let vault_path = base_dir.join("vault.json");
        let meta_path = base_dir.join("vault.meta");

        Self {
            vault_path: vault_path.to_string_lossy().into(),
            meta_path: meta_path.to_string_lossy().into(),
            key: [0u8; 32],
            salt: Vec::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let meta_path = Path::new(&self.meta_path);

        if !meta_path.exists() || fs::metadata(meta_path)?.len() == 0 {
            commands::init::init_vault(&self.meta_path)?;
        }
        
        let (key, salt) = authentication::authenticate(&self.meta_path, &self.vault_path)?;
        self.key = key;
        self.salt = salt;

        println!("Authenticated. Type `help` for options.");

        self.interactive_loop()
    }

    fn interactive_loop(&mut self) -> io::Result<()> {
        loop {
            print!("pwmgr> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let parts: Vec<&str> = input.trim().split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            let command = parts[0];
            let args = &parts[1..];

            match command {
                "add" if args.len() == 3 => {
                    commands::add::add_credentials(
                        &self.vault_path,
                        &self.key,
                        &self.salt,
                        args[0].to_string(),
                        args[1].to_string(),
                        args[2].to_string(),
                    )?;
                }
                "edit" if args.len() == 3 => {
                    commands::edit::edit_credential(
                        &self.vault_path,
                        &self.key,
                        &self.salt,
                        args[0].to_string(),
                        args[1].to_string(),
                        args[2].to_string(),
                    )?;
                }
                "get" if args.len() == 1 => {
                    commands::get::get_credential(
                        &self.vault_path, 
                        &self.key, 
                        args[0])?;
                }
                "delete" if args.len() == 1 => {
                    commands::delete::delete_credential(
                        &self.vault_path, 
                        &self.key, 
                        &self.salt, 
                        args[0])?;
                }
                "list" => {
                    commands::list::list_services(
                        &self.vault_path, 
                        &self.key)?;
                }
                "help" => {
                    println!("Commands:\n1. add <svc> <usr> <pwd>\n2. edit <svc> <usr> <pwd> \n3. get <svc>\n4. delete <svc>\n5. list\n6. exit");
                }
                "exit" => {
                    self.key.fill(0);
                    break;
                }
                _ => {
                    println!("Unknown command. Type `help`.");
                }
            }
        }
        Ok(())
    }
}
