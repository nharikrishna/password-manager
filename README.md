# Password Manager CLI

A lightweight, encrypted command-line password manager written in Rust. Stores your credentials locally and securely using AES-GCM and Argon2.

---

## Features

- Secure local storage (AES-GCM + Argon2)
- Add, retrieve, list, delete, and edit credentials
- All data encrypted with a master password
- Simple CLI interface — no internet required

---

## Installation

### With Cargo

```bash
cargo install --git https://github.com/nharikrishna/password-manager
```

After install, run:

```bash
password-manager
```
---

## Where Data Is Stored

Your encrypted vault is stored locally at:

```
~/.password-manager/vault.json
~/.password-manager/vault.meta
```
---

## Commands

Use the CLI interactively after authentication. Available commands may include:

```bash
add        # Add a new credential
get        # Retrieve a credential
list       # List all services
delete     # Delete a credential
edit       # Update username or password
help       # Show available commands
exit       # Quit the app
```

Example:

```bash
add
> Service: github
> Username: nharikrishna
> Password: ********
