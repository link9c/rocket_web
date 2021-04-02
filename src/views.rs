use rocket_contrib::json::{Json};
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use std::path::{PathBuf, Path};

use rocket::{State, response::Debug};
use rusqlite::{Error, types::ToSql};

use crate::common::{JsonReturn, JsonReceive, DbConn};


// 返回的json结构体


#[get("/db")]
pub fn db(db_conn: State<'_, DbConn>) -> Json<JsonReturn<String>> {
    let res: Result<String, Debug<Error>> = db_conn.lock()
        .expect("db connection lock")
        .query_row("SELECT name,id FROM entries WHERE id = 0",
                   &[] as &[&dyn ToSql], |row| { row.get(0) })
        .map_err(Debug);

    // println!("{:?}",res);

    // let now = time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap().to_string();
    // db_conn.lock().expect("run").
    //     execute("INSERT INTO counts (time,ip) VALUES ($1,$2)",
    //             &[now.as_str(), "0001"])
    //     .expect("insert single entry into entries table");

    let r = JsonReturn::<String>::new()
        .set_attr(1, vec![res.unwrap()], "success");
    Json(r)
}


// #[post("/add", data = "<js>")]
// pub fn add_one(js: Json<JsonReceive>) -> Json<JsonReturn<String>> {
//     let mut r = JsonReturn::<String>::new(0, "success".to_string());
//     r.data.push(js.data.to_string());
//     Json(r)
// }

#[post("/add", data = "<js>")]
pub fn add_one<'a>(js: Json<JsonReceive<'a>>) -> Json<JsonReturn<String>> {

    let r = JsonReturn::<String>::new()
        .set_attr(1, vec![js.data.to_string()], "success");
    Json(r)

}

//返回静态资源
#[get("/v1/<file..>")]
pub fn static_files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|_e| NotFound("404".to_string()))
}

