extern crate time;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};


use rocket::{Rocket, Request, State, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;
use rusqlite::Connection;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::ops::Deref;
use std::collections::hash_map::RandomState;

#[derive(Clone)]
pub struct Counter {
    pub get: Arc<AtomicUsize>,
    pub post: Arc<AtomicUsize>,
    pub conn: Arc<Mutex<Connection>>,
}

// #[rocket::async_trait]
impl Fairing for Counter {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Attach | Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request<'_>, _: &Data) {
        if request.method() == Method::Get {
            self.get.fetch_add(1, Ordering::Relaxed);
        } else if request.method() == Method::Post {
            self.post.fetch_add(1, Ordering::Relaxed);
        }


        // let ip = request.client_ip().unwrap().to_string();
        //
        // let now = time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();
        // self.conn.lock().expect("run").
        //     execute("INSERT INTO counts (time,ip) VALUES ($1,$2)",
        //             &[now.as_str(),ip.as_str()])
        //     .expect("insert single entry into entries table");
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        #[get("/counts")]
        fn counts(counts: State<'_, Counter>) -> String {
            let get_count = counts.get.load(Ordering::Relaxed);
            let post_count = counts.post.load(Ordering::Relaxed);

            format!("Get: {}\nPost: {}", get_count, post_count)
        }

        Ok(rocket.manage(self.clone()).mount("/", routes![counts]))
    }
}


pub struct Session {
    pub(crate) users: Mutex<HashMap<String, usize>>,
}

impl Session {
    pub fn contains_user(&self, other: &str) -> bool {
        match self.users.lock().unwrap().get(other) {
            None => false,
            Some(_) => true
        }
    }

    pub fn get_user(&self, other: &str) -> bool {
        match self.users.lock().unwrap().get(other) {
            None => false,
            Some(_) => true
        }
    }

    pub fn remove_user(&self, id: String) {
        let mut guard = self.users.lock().unwrap();
        guard.remove(id.as_str());
    }

    pub fn push_user(&self, id: String) {
        let mut guard = self.users.lock().unwrap();
        guard.insert(id, 1);
    }


}

///
///
/// custom session
///
pub type CusSession = Arc<Session>;