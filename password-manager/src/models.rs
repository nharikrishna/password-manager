use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Vault {
    pub credentials: HashMap<String, Credential>,
}