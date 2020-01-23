#[macro_use] extern crate failure;
extern crate rusqlite;
extern crate bcrypt;

use rusqlite::{Connection, NO_PARAMS};
use failure::{Error, err_msg};
use bcrypt::{DEFAULT_COST, hash, verify};

fn main() -> Result<(), Error> {
    let conn = Connection::open("database.db").unwrap();

    login("polly", "hunter2", &conn)?;

    Ok(())
}

fn create_database(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        r#"CREATE TABLE "users" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "username"	TEXT UNIQUE,
            "hash"	TEXT
        );"#,
        NO_PARAMS
    )?;

    Ok(())
}

fn register(username: &str, password: &str, conn: &Connection) -> Result<(), Error> {
    let hashed_password = hash(password, DEFAULT_COST)?;

    conn.execute(
        "INSERT INTO users (username, hash) VALUES (?1, ?2)",
        &[username, &hashed_password]
    )?;

    Ok(())
}

fn login(username: &str, password: &str, conn: &Connection) -> Result<bool, Error> {
    let mut statement = conn.prepare("SELECT hash FROM users WHERE username = ?1")?;
    let rows = statement.query_map(
        &[username],
        |row| row.get::<_, String>(0)
    )?;

    let hashed_password = rows.filter_map(Result::ok)
                              .next()
                              .ok_or(err_msg("No such user"))?;

    Ok(verify(password, &hashed_password)?)
}

