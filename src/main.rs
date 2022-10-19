mod database;
use database::KarmaDatabase;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
    let mut db = KarmaDatabase::new("my-database", "root", "root");
    let mut rl = Editor::<()>::new()?;
    Ok(loop {
        let readline = rl.readline("Karma/> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                db.execute(line.as_str());
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
