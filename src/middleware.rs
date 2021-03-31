extern crate time;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rocket::{Rocket, Request, State, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;
use rusqlite::Connection;

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
    pub id: Mutex<Vec<String>>,
}

impl Session {
    pub fn contains_user(&self, other: String) -> bool {
        self.id.lock().unwrap().contains(&other)
    }

    pub fn remove_user(&mut self, id: String) {
        let mut guard = self.id.lock().unwrap();
        guard.remove(0);
    }

    pub fn push_user(&self, id: String) {
        let mut guard = self.id.lock().unwrap();
        guard.push(id)
    }
}

///
///
/// custom session
///
pub type CusSession = Arc<Session>;