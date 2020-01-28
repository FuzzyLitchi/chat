use rusqlite::{Connection, NO_PARAMS};
use failure::{Error, err_msg};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;

use super::types::{Message, User};

pub fn create_database(conn: &Connection) -> Result<(), Error> {
    conn.execute(r#"
        CREATE TABLE "users" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "username"	TEXT UNIQUE,
            "hash"	TEXT
        );"#,
        NO_PARAMS
    )?;
    conn.execute(r#"
        CREATE TABLE "messages" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "sender"	INTEGER NOT NULL,
            "recipient"	INTEGER NOT NULL,
            "message"	TEXT NOT NULL,
            "datetime"	TIMESTAMP NOT NULL
        )
        "#,
        NO_PARAMS
    )?;

    Ok(())
}

pub fn register(username: &str, password: &str, conn: &Connection) -> Result<(), Error> {
    let hashed_password = hash(password, DEFAULT_COST)?;

    conn.execute(
        "INSERT INTO users (username, hash) VALUES (?1, ?2)",
        &[username, &hashed_password]
    )?;

    Ok(())
}

pub fn login(username: &str, password: &str, conn: &Connection) -> Result<u32, Error> {
    let mut statement = conn.prepare("SELECT hash, id FROM users WHERE username = ?1")?;
    let rows = statement.query_map(
        &[username],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u32>(1)?
            ))
        }
    )?;

    let (hashed_password, id) = rows.filter_map(Result::ok)
                              .next()
                              .ok_or(err_msg("No such user"))?;

    if verify(password, &hashed_password)? {
        Ok(id)
    } else {
        Err(err_msg("Incorrect password"))
    }
}

pub fn get_users(conn: &Connection) -> Result<Vec<User>, Error> {
    let mut statement = conn.prepare("SELECT id, username FROM users")?;
    let rows = statement.query_map(
        NO_PARAMS,
        |row| {
            Ok(User::new(
                row.get::<_, u32>(0)?,
                row.get::<_, String>(1)?,
            ))
        }
    )?;

    Ok(rows.flatten().collect())
}

pub fn send_message(from: u32, to: u32, message: &str, conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO messages (sender, recipient, message, datetime) VALUES (?1, ?2, ?3, ?4)",
        params![from, to, message, Utc::now().timestamp()]
    )?;

    Ok(())
}

pub fn get_messages(id: u32, conn: &Connection) -> Result<Vec<Message>, Error> {
    let mut statement = conn.prepare(r#"
        SELECT id, sender, recipient, message, datetime
        FROM messages
        WHERE recipient = ?1"#
    )?;

    let rows = statement.query_map(
        &[id],
        |row| {
            Ok(Message::new(
                row.get::<_, u32>(0)?,
                row.get::<_, u32>(1)?,
                row.get::<_, u32>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        }
    )?;

    // Only keep Ok()
    let messages: Vec<Message> = rows.flatten().collect();

    Ok(messages)
}