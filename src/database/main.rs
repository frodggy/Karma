use std::{
    collections::HashMap,
    fs,
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
            file: format!("{}.kdb", name),
        }
    }

    pub async fn get(&self, key: &str) /* -> Option<KarmaStore> */ {
        let db = self.db.lock().unwrap();
        // match db.get(key).cloned() {
        //     Some(item) => return Some(item),
        //     None => return None
        // }
        println!("{:?}", db.get(key).cloned());
    }
    pub async fn set(&self, key: &str, value: KarmaStore) {
        let mut db = self.db.lock().unwrap();
        let out = db.insert(String::from(key), value);

    }
    pub async fn delete(&self, key: &str) {
        let mut db = self.db.lock().unwrap();
        let out = db.remove(key);
        if let None = out {
            eprintln!("deletion failed")
        }
    }
    pub async fn save_to_kdb(&self) {
        let buffer = bincode::serialize(&self).unwrap();
        let _ = fs::write(format!("/etc/karmadb/{}", self.file), buffer).unwrap();
    }
    pub async fn load_from_kdb(&mut self, file: String) {
        let buffer = fs::read(format!("/etc/karmadb/{}", file)).expect("Unable to read file");
        let data = bincode::deserialize::<KarmaDatabase>(&buffer[..]).unwrap();
        self.db = data.db;
        self.file = data.file;
        self.name = data.name;
        self.username = data.username;
        self.password = data.password;
        self.port = data.port;
    }

    pub async fn execute(&mut self, query: &str) {
        let mut q = query.split_whitespace().collect::<Vec<&str>>();
        match q[0] {
            "get" => {
                if q.len() == 2 {
                    self.get(q[1]).await;
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            "set" => {
                if q.len() == 3 {
                    if q[2].starts_with("%n") {
                        q[2] = q[2].strip_prefix("%n").unwrap();
                        self.set(q[1], KarmaStore::Int(q[2].parse().unwrap())).await;
                    } else {
                        self.set(q[1], KarmaStore::String(q[2].to_string())).await;
                    }
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            "delete" => {
                if q.len() == 2 {
                    self.delete(q[1]).await;
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            "help" => {
                println!("commands:\n   set - set key to a value\n   get - retrieve value by key\n   delete - delete value by key\n   help - prints commands"
                )
            }
            ".write" => self.save_to_kdb().await,
            ".open" => {
                if q.len() == 2 {
                    self.load_from_kdb(format!("{}.kdb", String::from(q[1]))).await
                } else {
                    eprintln!("Not enough arguments")
                }
            }
            &_ => eprintln!("Unrecognized operation"),
        }
    }
}
