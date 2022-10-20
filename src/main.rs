// local://database
mod database;
use database::{KarmaDatabase, KarmaStore};
// std
use std::env;
use std::process::exit;

// crates://rustyline
use rustyline::error::ReadlineError;
use rustyline::Editor;

// crates://tokio
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// crates://slice_as_array
#[macro_use]
extern crate slice_as_array;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let db = KarmaDatabase::new("my-database", "root", "root");
    match args[1].as_str() {
        "repl" => {
            let _ = repl(db).await.unwrap();
        }
        "server" => server(db).await,
        _ => exit(1),
    }
}

async fn repl(mut db: KarmaDatabase) -> rustyline::Result<()> {
    let mut rl = Editor::<()>::new()?;
    Ok(loop {
        let readline = rl.readline("Karma/> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                db.execute(line.as_str()).await;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    })
}

async fn server(database: KarmaDatabase) {
    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", database.port))
        .await
        .unwrap();
    println!("Server listening on port {}", database.port);
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let db = database.clone();

        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];

            let n: usize = match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading socket: {}", e);
                    return;
                }
            };

            let operation_buffer =
                slice_as_array!(&buf[0..4], [u8; 4]).expect("failed to read operation");
            let operation = i32::from_be_bytes(*operation_buffer);

            let data_type_buffer =
                slice_as_array!(&buf[4..8], [u8; 4]).expect("failed to read data type");
            let data_type = i32::from_be_bytes(*data_type_buffer);

            let key_buffer = slice_as_array!(&buf[8..28], [u8; 20]).expect("failed to read key");
            let key = String::from_utf8_lossy(key_buffer).to_string();

            let val: KarmaStore;

            match data_type {
                1 => {
                    let value_buffer =
                        slice_as_array!(&buf[28..428], [u8; 400]).expect("failed to read value");
                    val = KarmaStore::String(String::from_utf8(value_buffer.to_vec()).unwrap());
                }

                2 => {
                    let value_buffer =
                        slice_as_array!(&buf[28..32], [u8; 4]).expect("failed to read value");
                    val = KarmaStore::Int(i32::from_be_bytes(*value_buffer));
                }, 

                i32::MIN..=0_i32 | 3_i32..=i32::MAX => {
                    eprintln!("invalid operation");
                    return;
                }
            }
            match operation {
                1 => {
                    println!("{}:{:?}", key, val);
                    db.set(&key, val).await;
                    db.save_to_kdb().await;
                    println!("write complete")
                },
                2 => {
                    db.get(&key).await;
                    db.save_to_kdb().await;
                    println!("read complete")
                },
                3 => {
                    db.delete(&key).await;
                    db.save_to_kdb().await;
                    println!("delete complete")
                },
                i32::MIN..=0_i32 | 4_i32..=i32::MAX => {
                    eprintln!("invalid operation");
                    return;
                }
            }

            if let Err(e) = socket.write_all(&buf[0..n]).await {
                eprintln!("Error writing to socket: {:?}", e);
            }
        });
    }
}
