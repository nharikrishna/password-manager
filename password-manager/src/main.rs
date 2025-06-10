mod models;
mod storage;
mod crypto;
mod commands;
mod authentication;
mod app;

fn main() -> std::io::Result<()> {
    let vault_path = "vault.json";
    let meta_path = "vault.meta";

    let mut app = app::PasswordManagerApp::new(vault_path, meta_path);
    app.run()
}