use rocket_contrib::json::{JsonError, Json};
use rocket::State;
use rusqlite::Error;
use serde::{Deserialize};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::common::{DbConn, JsonReturn};

#[derive(Deserialize, Clone)]
pub struct RegisterInfo {
    username: String,
    email: String,
    password: String,
}

// code
// 5eca150eb854887ca100

#[post("/register", data = "<reg>")]
pub fn do_register(reg: Result<Json<RegisterInfo>, JsonError<'_>>,
                   db: State<DbConn>) -> Json<JsonReturn<'static, String>> {
    let r = JsonReturn::<String>::new();

    match reg {
        Ok(js) => {
            let status: Result<i32, Error> = db.lock().unwrap()
                .query_row("select status from  users where name =$1 or email =$2",
                           &[js.username.as_str(),js.email.as_str()],
                           |row| row.get(0) );
            match status {
                Ok(_) => {
                    return Json(r.set_attr(0, vec![], "account have exist"));
                }
                Err(e) => {
                    db.lock().unwrap().execute(
                        "insert into  users (name,password,email,status) values ($1,$2,$3,$4)"
                        , &[js.username.as_str(), js.password.as_str(), js.email.as_str(), "1"],
                    ).expect("insert error");

                    // let email = Message::builder()
                    //     .from("GameHistory <GH@domain.com>".parse().unwrap())
                    //     .reply_to(js.email.as_str().parse().unwrap())
                    //     .to(js.email.parse().unwrap())
                    //     .subject("验证邮件 请勿回复")
                    //     .body(String::from("请在30分钟内点击下面链接验证账号"))
                    //     .unwrap();

                    // let email = Message::builder()
                    //     .from("NoBody <nobody@domain.tld>".parse().unwrap())
                    //     .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
                    //     .to("Hei <hei@domain.tld>".parse().unwrap())
                    //     .subject("Happy new year")
                    //     .body(String::from("Be happy!"))
                    //     .unwrap();
                    //
                    // let creds = Credentials::new(
                    //     "18762989218@139.com".to_string()
                    //     , "5eca150eb854887ca100".to_string());
                    //
                    //
                    // let mailer = SmtpTransport::starttls_relay("smtp.139.com")
                    //     .unwrap()
                    //     .credentials(creds)
                    //     .build();
                    //
                    // match mailer.send(&email) {
                    //     Ok(_) => println!("Email sent successfully!"),
                    //     Err(e) => panic!("Could not send email: {:?}", e),
                    // }

                    return Json(r.set_attr(0, vec![]
                                           , "account create success.please valid it by mail"));
                }
            }

        }
        Err(_) => {
            Json(r.set_attr(1, vec![], "Invalid Fields"))
        }
    }
}



