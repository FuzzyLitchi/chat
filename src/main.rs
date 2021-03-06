extern crate failure;
#[macro_use] extern crate rusqlite;
extern crate bcrypt;
#[macro_use] extern crate text_io;
extern crate chrono;

mod database;
mod types;

use std::io::{stdout, Write};
use std::collections::HashMap;
use failure::Error;
use rusqlite::Connection;

use database::{login, get_messages, get_users, send_message};
use types::User;

fn main() -> Result<(), Error> {
    let conn = Connection::open("database.db")?;

    let mut users: HashMap<u32, String> = HashMap::new();
    
    for user in get_users(&conn)? {
        users.insert(user.id, user.username);
    }
    
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
            Ok(1) => {
                let messages = get_messages(user.id, &conn)?;

                for message in messages {
                    let sender = users.get(&message.sender)
                                      .map(String::as_str)
                                      .unwrap_or("Unknown");
                    
                    println!("From: {}", sender);
                    println!("Date: {}", message.datetime);
                    println!("\n{}", message.message);
                }
            },
            Ok(2) => send_message_cli(&user, &users, &conn)?,
            Ok(3) => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Error, please choose one of the listed options by writing a number from 1-3"),
        }
    }

    Ok(())
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

fn send_message_cli(user: &User, users: &HashMap<u32, String>, conn: &Connection) -> Result<(), Error> {
    let recipient: u32;
    
    loop {
        println!("Select one of the following user ids");
        for (id, username) in users.iter() {
            println!("{} -> {}", id, username);
        }
        stdout().flush()?;

        let choice: Result<u32, _> = try_read!("{}\n");
        if let Ok(choice) = choice {
            if users.contains_key(&choice) {
                recipient = choice;
                break;
            }

        }
    }

    println!("Compose message, end by writing \"END\" by itself on a new line");

    let mut message = String::new();
    
    loop {
        let line: String = read!("{}\n");
        if line == "END" {
            break
        } else {
            message.push_str(&line);
            message.push('\n');
        }
    }

    send_message(user.id, recipient, &message, conn)
}