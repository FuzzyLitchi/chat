use chrono::NaiveDateTime;

pub struct User {
    pub id: u32,
    pub username: String,
}

impl User {
    pub fn new(id: u32, username: String) -> Self {
        Self {
            id,
            username
        }
    }
}

pub struct Message {
    pub id: u32,
    pub sender: u32,
    pub recipient: u32,
    pub message: String,
    pub datetime: NaiveDateTime,
}

impl Message {
    pub fn new(id: u32, sender: u32, recipient: u32, message: String, datetime: i64) -> Self {
        Message {
            id,
            sender,
            recipient,
            message,
            datetime: NaiveDateTime::from_timestamp(datetime, 0),
        }
    }
}