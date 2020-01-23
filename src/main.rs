#[macro_use] extern crate failure;
extern crate rusqlite;
extern crate bcrypt;
#[macro_use] extern crate text_io;

mod database;

use std::io::Write;
use std::io::stdout;
use failure::Error;
use rusqlite::Connection;

use database::login;

fn main() -> Result<(), Error> {
    let conn = Connection::open("database.db")?;

    let user: User;

    loop {
        if let Some(u) = login_cli(&conn) {
            user = u;
            break
        }
    }

    // Do messaging cli

    Ok(())
}

struct User {
    id: u64,
    username: String,
}

impl User {
    fn new(id: u64, username: String) -> Self {
        Self {
            id,
            username
        }
    }
}

fn login_cli(conn: &Connection) -> Option<User> {
    print!("Username: ");
    stdout().flush().unwrap();
    let username: String = read!();

    print!("Password: ");
    stdout().flush().unwrap();
    let password: String = read!();

    return match login(&username, &password, conn) {
        Ok(true) => {
            println!("Logged into {}!", username);
            Some(User::new(0, username))
        },
        Ok(false) => {
            println!("Wrong password");
            None
        },
        Err(err) => {
            println!("Error occured: {}", err);
            None
        }
    }
}