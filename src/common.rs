use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use std::sync::{Mutex, Arc};


#[derive(Serialize)]
pub struct JsonReturn<'a, T> {
    pub(crate) code: i32,
    pub(crate) data: Vec<T>,
    pub(crate) message: &'a str,
    timestamp: String,
}

impl<'a, T> JsonReturn<'a, T> {
    pub fn new(code: i32, message: &'a str) -> JsonReturn<'a, T> {
        let now = time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();
        JsonReturn { code: code, data: Vec::new(), message: message, timestamp: now }
    }
}

#[derive(Deserialize, Clone)]
pub struct JsonReceive<'a> {
    code: i32,
    pub(crate) data: &'a str,
}

pub type DbConn = Arc<Mutex<Connection>>;


