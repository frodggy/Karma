mod database;
use database::KarmaDatabase;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let db = KarmaDatabase::new("my-database", "root", "root");
    let mut rl = Editor::<()>::new()?;
    Ok(loop {
        let readline = rl.readline("Karma/> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.as_str() {
                    "help" => {
                        println!("commands:\n   set - set key to a value\n   get - retrieve value by key\n   delete - delete value by key\n   help - prints commands"
                        )
                    }
                    _ => db.execute(line.as_str()),
                }
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
