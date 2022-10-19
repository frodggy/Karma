use std::{sync::{Arc, Mutex}, collections::HashMap};


#[derive(Debug, Clone)]
pub enum KarmaStore {
    String(String),
    Int(i32)
}

#[derive(Debug)]
pub struct KarmaDatabase {
    pub db: Arc<Mutex<HashMap<String, KarmaStore>>>,
    pub name: String,
    pub username: String,
    pub password: String,
    pub port: i32,
}