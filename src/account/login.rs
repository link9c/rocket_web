use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest,  Outcome};
use rocket::http::{Cookies, Cookie};
use rocket::{Request,  State};
use serde::{Deserialize};
use rocket_contrib::json::{Json, JsonError};
use crate::middleware::CusSession;
use crate::common::{JsonReturn};


// use rocket::http::private::CookieJar;


#[derive(Deserialize, Clone)]
pub struct Login {
    username: String,
    password: String,
}

// todo database转为vec 保存多个用户 共享用户信息
pub struct Database;

impl Database {
    fn set_user(&self, id: String) -> Result<User, ()> {
        Ok(User { id, is_admin: false })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Database {
    type Error = ();
    fn from_request(_request: &'a Request<'r>) -> request::Outcome<Database, ()> {
        Outcome::Success(Database)
    }
}


pub struct User { id: String, is_admin: bool }

// 单次缓存
impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, ()> {
        println!("in user FromRequest");
        let p = request.uri().path();
        let user_result: &Option<User> = request.local_cache(|| {
            let db = request.guard::<Database>().succeeded()?;
            if p.contains("/dash") {
                println!("set id");
                request.cookies()
                    .get_private("user_id")
                    .and_then(|cookie| cookie.value().parse().ok())
                    .and_then(|id| db.set_user(id).ok())

                // db.set_user("guest2".to_string()).ok()
            } else {
                // Outcome::Forward(())
                db.set_user(String::default()).ok()
            }
        });

        user_result.as_ref().or_forward(())
    }
}

pub struct Admin<'a> { user: &'a User }

impl<'a, 'r> FromRequest<'a, 'r> for Admin<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin<'a>, ()> {
        println!("in admin FromRequest");
        // let a  = request.guard::<State<DbConn>>();
        // a.unwrap().lock()
        //     .expect("aaa")
        //     .execute("INSERT INTO counts (time,ip) VALUES ($1,$2)",&["dd", "0001"])
        //     .expect("error");
        //             &[now.as_str(), "0001"])
        let user = request.guard::<&User>()?;

        if user.is_admin {
            Outcome::Success(Admin { user })
        } else {
            Outcome::Forward(())
        }
    }
}


#[get("/dash", rank = 2)]
pub fn admin_dashboard(_admin: Admin) -> String {
    // admin.user.id
    "kkkk".to_string()
}

#[get("/dash", rank = 4)]
pub fn user_dashboard(_user: &User) -> String {
    // user.id
    "uuuuu".to_string()
}


#[post("/login", data = "<login>")]
pub fn login(login: Result<Json<Login>, JsonError<'_>>, session: State<CusSession>, mut cookies: Cookies,
) -> Json<JsonReturn<'static, String>> {
    let mut r = JsonReturn::<String>::new(0, "success");
    match login {
        Ok(t) => {
            if t.username == "Sergio" && t.password == "password" {
                session.push_user("Sergio".to_string());
                cookies.add_private(Cookie::new("user_id", "io"));
                // cookies.add_private(Cookie::new("user_id", 1.to_string()));
                Json(r)
            } else {
                r.code = 1;
                r.message = "Invalid username/password.";
                Json(r)
            }
        }
        Err(_m) => {
            r.code = 1;
            // type Error = serde_json::error::Error;
            r.message = "Invalid Fields";
            Json(r)
        }
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies, session: State<CusSession>) -> Json<JsonReturn<'static, String>> {
    cookies.remove_private(Cookie::named("user_id"));
    let f = session.id.lock().unwrap();
    println!("{:?}", f);
    let r = JsonReturn::<String>::new(0, "Successfully logged out.");
    Json(r)
}



