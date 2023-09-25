//DO NOT REMOVE
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, Result, Error};
use std::borrow::Cow;

#[derive(Debug)]
struct Command {
    name: String,
    midBalls: u8,
    allCost: f32,
}

#[derive(Debug)]
struct Player {
    NLP: String,
    balls: u8,
    contractCost: f32,
    command: String,
}

#[tauri::command]
fn get_input_player(NLP: String, balls: u8, contractCost: f32, command: String){
    let person = player_to_struct(NLP, balls, contractCost, command);
    import_to_db_player(person);
}

fn parse_db() -> Result<(), Error> {
    let resoult = dbconnect();
    let conn = match resoult {
        Ok(value) => value,
        Err(_) => {
            println!("Error");
            return Err(Error::SqliteFailure(rusqlite::ffi::Error::new(1), None));
        }
    };

    let mut stmt = conn.prepare("SELECT NLP, balls, contractCost, command FROM Player")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Player {
            NLP: row.get(0)?,
            balls: row.get(1)?,
            contractCost: row.get(2)?,
            command: row.get(3)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}

fn player_to_struct(NLP: String, balls: u8, contractCost: f32, command: String) -> Player {
    let person = Player {
        NLP: NLP,
        balls: balls,
        contractCost: contractCost,
        command: command,
    };

    return person;
}

//unrelised fn
fn import_to_db_command(command: Command) -> Result<(), Error>{
    let resoult = dbconnect();
    let conn = match resoult {
        Ok(value) => value,
        Err(_) => {
            println!("Error");
            return Err(Error::SqliteFailure(rusqlite::ffi::Error::new(1), None));
        }
    };
    conn.execute(
        "INSERT INTO Players (name, midBalls, allCost) VALUES (?, ?, ?)",
        (command.name.to_owned(), command.midBalls, command.allCost),
    )?;
    Ok(())
}

fn import_to_db_player(player: Player) -> Result<(), Error>{
    let resoult = dbconnect();
    let conn = match resoult {
        Ok(value) => value,
        Err(_) => {
            println!("Error");
            return Err(Error::SqliteFailure(rusqlite::ffi::Error::new(1), None));
        }
    };
    conn.execute(
        "INSERT INTO Players (NLP, balls, contractCost, command) VALUES (?, ?, ?, ?)", 
        (player.NLP.to_string(), player.balls, player.contractCost, player.command.to_string()),
    )?;
    Ok(())
}

fn dbconnect() -> Result<Connection, Error> {
    let conn = Connection::open("roomcommand.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Players (
        NLP TEXT,
        balls INT,
        contractCost REAL,
        command TEXT
      )",
      (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Commands (
        name TEXT,
        midBalls INT,
        allCost REAL
      )",
      (),
    )?;
    Ok(conn)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_input_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
