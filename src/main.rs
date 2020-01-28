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

    // Login loop
    loop {
        if let Some(u) = login_cli(&conn) {
            user = u;
            break
        }
    }

    // Cli interface
    loop {
        println!(r#"
Choose one of following actions
1) Check incoming messages
2) Send message
3) Log out"#);
        stdout().flush().unwrap();

        let choice: Result<u8, _> = try_read!("{}\n");
        match choice {
            Ok(1) => println!("1"),
            Ok(2) => println!("2"),
            Ok(3) => println!("3"),
            _ => println!("Error, please choose one of the listed options by writing a number from 1-3"),
        }
    }

    Ok(())
}

struct User {
    id: u32,
    username: String,
}

impl User {
    fn new(id: u32, username: String) -> Self {
        Self {
            id,
            username
        }
    }
}

fn login_cli(conn: &Connection) -> Option<User> {
    print!("Username: ");
    stdout().flush().unwrap();
    let username: String = read!("{}\n");

    print!("Password: ");
    stdout().flush().unwrap();
    let password: String = read!("{}\n");

    return match login(&username, &password, conn) {
        Ok(id) => {
            println!("Logged into {}!", username);
            Some(User::new(id, username))
        },
        Err(err) => {
            println!("Error occured: {}", err);
            None
        }
    }
}