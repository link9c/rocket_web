#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]
#[macro_use]
extern crate rocket;


mod views;
mod middleware;
mod account;
mod common;


use views::*;
use middleware::*;
use account::login::*;
use std::sync::{Arc, Mutex};
use rocket::fairing::AdHoc;
use rusqlite::Connection;
use std::cell::RefCell;


const DBPATH: &'static str = "D:/Program Files (x86)/sqlite3/rocket_admin";

// pub fn init_database(conn: &Connection) {
//     conn.execute("CREATE TABLE entries (
//                   id              INTEGER PRIMARY KEY,
//                   name            TEXT NOT NULL
//                   )", &[] as &[&dyn ToSql])
//         .expect("create entries table");
//
//     conn.execute("INSERT INTO entries (id, name) VALUES ($1, $2)",
//                  &[&0 as &dyn ToSql, &"Rocketeer"])
//         .expect("insert single entry into entries table");
// }


fn main() {
    let conn = Arc::new(
        Mutex::new(
            Connection::open(DBPATH).expect("not find")
        )
    );
    let sess = Arc::new(Session { users: Mutex::default() });

    let count = Counter {
        get: Arc::new(Default::default()),
        post: Arc::new(Default::default()),
        conn: conn.clone(),
    };
    rocket::ignite()
        .mount("/", routes![
        // index,
        static_files,
        add_one,
        db,
        login,
        logout,

        ])
        .attach(count)
        .attach(AdHoc::on_launch("Launch Message", |_| {
            println!("Rocket is about to launch!");
        }))
        .manage(conn)
        .manage(sess)
        .launch();
}