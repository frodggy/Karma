use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{types::KarmaDatabase, KarmaStore};

impl KarmaDatabase {
    pub fn new(name: &str, username: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            port: 9990,
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> KarmaStore {
        let db = self.db.lock().unwrap();
        return db.get(key).unwrap().clone();
    }

    pub fn set(&self, key: &str, value: KarmaStore) {
        let mut db = self.db.lock().unwrap();
        db.insert(String::from(key), value);
        println!("{:?}", db)
    }

    pub fn delete(&self, key: &str) {
        let mut db = self.db.lock().unwrap();
        db.remove(key);
        println!("{:?}", db)
    }

    pub fn execute(&self, query: &str) {
        let mut q = query.split_whitespace().collect::<Vec<&str>>();
        match q[0] {
            "get" => {
                if q.len() == 2 {
                    let value = self.get(q[1]);
                    println!("{:?}", value);
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            "set" => {
                if q.len() == 3 {
                    if q[2].starts_with("%n") {
                        q[2] = q[2].strip_prefix("%n").unwrap();
                        self.set(q[1], KarmaStore::Int(q[2].parse().unwrap()));
                    } else {
                        self.set(q[1], KarmaStore::String(q[2].to_string()));
                    }
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            "delete" => {
                if q.len() == 2 {
                    self.delete(q[1]);
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            &_ => eprintln!("Unrecognized operation"),
        }
    }
}
