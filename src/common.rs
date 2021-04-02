use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use std::sync::{Mutex, Arc};
use std::borrow::Borrow;


#[derive(Serialize)]
pub struct JsonReturn<'a, T> {
    pub(crate) code: i32,
    pub(crate) data: Vec<T>,
    pub(crate) message: &'a str,
    timestamp: String,
}

impl<'a, T> JsonReturn<'a, T> {
    pub fn new() -> Self {
        let now = time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();
        JsonReturn { code: 0, data: Vec::new(), message: "success", timestamp: now }
    }
    pub fn set_attr(mut self, code: i32, data: Vec<T>, msg: &'a str) -> JsonReturn<'a, T> {
        self.code = code;
        self.data = data;
        self.message = msg;
        self
    }
}

#[derive(Deserialize, Clone)]
pub struct JsonReceive<'a> {
    code: i32,
    pub(crate) data: &'a str,
}

pub type DbConn = Arc<Mutex<Connection>>;


