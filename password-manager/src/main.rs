mod models;
mod storage;
mod crypto;
mod commands;
mod authentication;
mod app;

fn main() -> std::io::Result<()> {
    let mut app = app::PasswordManagerApp::new();
    app.run()
}