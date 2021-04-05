use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Outcome};
use rocket::http::{Cookies, Cookie};
use rocket::{Request, State};
use serde::{Deserialize};
use rocket_contrib::json::{Json, JsonError};
// use rocket_contrib::json;
use crate::middleware::CusSession;
use crate::common::{JsonReturn, DbConn};
use rusqlite::Error;


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


pub struct User { pub id: String, pub is_admin: bool }

// 单次缓存
impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, ()> {
        println!("in user FromRequest");
        let p = request.uri().path();

        let sess = request.guard::<State<CusSession>>().succeeded().expect("error");
        let user_result = request.local_cache(|| {
            let db = request.guard::<Database>().succeeded().expect("error");
            if p.contains("/dash") {
                println!("set id");
                let cookie = request.cookies().get_private("user_id");
                if let Some(t) = cookie {
                    let temp_id = t.to_string();
                    let id = temp_id.split("=").collect::<Vec<&str>>()[1];
                    match sess.get_user(id) {
                        false => { return Err(()); }
                        true => {
                            return Ok(db.set_user(id.to_string()).unwrap());
                        }
                    }
                }
                Err(())
                // Outcome::Forward(())
            } else {
                Ok(db.set_user("guest".to_string()).unwrap())
                // Outcome::Success(user_result)
            }
        });
        match user_result.as_ref() {
            Ok(t) => { Outcome::Success(t) }
            Err(_) => { Outcome::Forward(()) }
        }
    }
}

pub struct LoginIP(String);

impl<'a, 'r> FromRequest<'a, 'r> for LoginIP {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoginIP, ()> {
        let ip = request.remote().expect("connect error").ip().to_string();

        Outcome::Success(LoginIP(ip))
    }
}


#[post("/login", data = "<login>")]
pub fn login(
    login: Result<Json<Login>, JsonError<'_>>,
    session: State<CusSession>,
    db: State<DbConn>,
    mut cookies: Cookies,
    ip: LoginIP,
) -> Json<JsonReturn<'static, String>> {
    // let mut r = JsonReturn::<String>::new(0, "success");
    let r = JsonReturn::<String>::new();
    match login {
        Ok(t) => {
            let username = t.username.clone();
            let status: Result<i32, Error> = db.lock().expect("connection lock")
                .query_row("select status from  users where name =$1 and password =$2 or email =$1 and password= $2"
                           , &[username.as_str(), t.password.as_str()]
                           , |row| { row.get(0) });

            match status {
                Ok(st) => {
                    println!("{} status {}", username.as_str(), st);
                    let now = time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();

                    db.lock().unwrap().execute(
                        "insert into login_log (login_ip,user_id,login_time) values ($1,$2,$3)",
                        &[ip.0.as_str(), username.as_str(), now.as_str()],
                    ).expect("insert error");
                    session.push_user(username.clone());
                    cookies.add_private(Cookie::new("user_id", username));


                    return Json(r.set_attr(0, vec![], "login successfully"));
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    return Json(r.set_attr(1, vec![], "Invalid username/password."));
                }
            }
        }
        Err(_m) => {
            Json(r.set_attr(1, vec![], "Invalid Fields"))
        }
    }
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies, mut session: State<CusSession>) -> Json<JsonReturn<'static, String>> {
    let cookie = cookies.get_private("user_id");
    // sess.lock().unwrap().remove_user(c);
    let r = JsonReturn::<String>::new();

    println!("{:?}", session.clone().users);
    match cookie {
        None => {
            Json(r.set_attr(1, vec![], "you have logged out"))
        }
        Some(t) => {
            let temp_id = t.to_string();
            let id = temp_id.split("=").collect::<Vec<&str>>()[1];
            println!("{}", id);
            session.remove_user(id.to_string());
            cookies.remove_private(Cookie::named("user_id"));

            Json(r)
        }
    }
}



