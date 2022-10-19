use std::{sync::{Arc, Mutex}, collections::HashMap};

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KarmaStore {
    String(String),
    Int(i32)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KarmaDatabase {
    pub db: Arc<Mutex<HashMap<String, KarmaStore>>>,
    pub file: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub port: i32,
}