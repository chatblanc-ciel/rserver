use rserver::web_dealer::get_service;
use rserver::web_dealer::http_def::*;
use rserver::web_dealer::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate tera;

mod model;
mod schema;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
//use std::str;
use schema::memos;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    content: String,
}

fn form(
    request: HttpRequest,
    _: String,
    pool: Option<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: Option<Tera>,
) -> Result<HttpResponse, HttpError> {
    let mut ctx = Context::new();
    let conn = pool
        .ok_or(HttpError::InternalErrorDiesel)?
        .get()
        .expect("couldn't get db connection from pool");
    let memos = memos::table
        .load::<crate::model::Memo>(&conn)
        .expect("Error loading cards");
    ctx.insert("memos", &memos);
    let view = tmpl
        .ok_or(HttpError::InternalErrorTera)?
        .render("form.tera", &ctx)
        .map_err(|_| HttpError::InternalErrorTera)?;
    Ok(HttpResponse {
        state: HttpResponseState::Complete,
        ver: request.ver,
        body: view,
    })
}

fn memo_form(
    request: HttpRequest,
    _: String,
    pool: Option<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: Option<Tera>,
) -> Result<HttpResponse, HttpError> {
    let content = request.body.trim().split("=").collect::<Vec<&str>>();
    let new_memo = crate::model::NewMemo {
        content: content
            .get(1)
            .ok_or(HttpError::RequestIsBroken)?
            .to_string(),
    };
    let conn = pool
        .ok_or(HttpError::InternalErrorDiesel)?
        .get()
        .expect("couldn't get db connection from pool");
    diesel::insert_into(memos::table)
        .values(&new_memo)
        .execute(&conn)
        .unwrap();
    let mut ctx = Context::new();
    let memos = memos::table
        .load::<crate::model::Memo>(&conn)
        .expect("Error loading cards");
    ctx.insert("memos", &memos);
    let view = tmpl
        .ok_or(HttpError::InternalErrorTera)?
        .render("form.tera", &ctx)
        .map_err(|_| HttpError::InternalErrorTera)?;
    Ok(HttpResponse {
        state: HttpResponseState::Complete,
        ver: request.ver,
        body: view,
    })
}

fn memo_delete(
    request: HttpRequest,
    _: String,
    pool: Option<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: Option<Tera>,
) -> Result<HttpResponse, HttpError> {
    // parse request URL
    let target = request.target.trim().split("?").collect::<Vec<&str>>();
    let parse_url = target
        .get(0)
        .ok_or(HttpError::RequestIsBroken)?
        .split("/")
        .collect::<Vec<&str>>();
    let table_id: i32 = parse_url
        .last()
        .ok_or(HttpError::RequestIsBroken)?
        .to_string()
        .trim()
        .parse()
        .or(Err(HttpError::RequestIsBroken))?;

    // delete database item
    let conn = pool
        .as_ref()
        .ok_or(HttpError::InternalErrorDiesel)?
        .get()
        .expect("couldn't get db connection from pool");
    diesel::delete(memos::table)
        .filter(memos::id.eq(table_id))
        .execute(&conn)
        .or(Err(HttpError::InternalErrorDiesel))?;

    Ok(HttpResponse {
        state: HttpResponseState::Complete,
        ver: request.ver,
        body: String::new(),
    })
}

fn memo_update(
    request: HttpRequest,
    _: String,
    pool: Option<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: Option<Tera>,
) -> Result<HttpResponse, HttpError> {
    // parse request URL
    let target = request.target.trim().split("?").collect::<Vec<&str>>();
    let parse_url = target
        .get(0)
        .ok_or(HttpError::RequestIsBroken)?
        .split("/")
        .collect::<Vec<&str>>();
    let table_id: i32 = parse_url
        .last()
        .ok_or(HttpError::RequestIsBroken)?
        .to_string()
        .trim()
        .parse()
        .or(Err(HttpError::RequestIsBroken))?;
    let content = request.body.trim().split("=").collect::<Vec<&str>>();
    let content = content
        .get(1)
        .ok_or(HttpError::RequestIsBroken)?
        .to_string();

    // delete database item
    let conn = pool
        .as_ref()
        .ok_or(HttpError::InternalErrorDiesel)?
        .get()
        .expect("couldn't get db connection from pool");

    diesel::update(memos::table)
        .filter(memos::id.eq(table_id))
        .set(memos::content.eq(content))
        .execute(&conn)
        .or(Err(HttpError::InternalErrorDiesel))?;

    Ok(HttpResponse {
        state: HttpResponseState::Complete,
        ver: request.ver,
        body: String::new(),
    })
}

fn main() {
    let templates = Tera::new("templates/**/*").unwrap();

    let database_url = "DATABASE_URL".to_owned();

    dotenv().ok();
    let dotenv_database_url = std::env::var(database_url).expect("DATABASE_URL must be set");
    let db_pool = r2d2::Pool::builder()
        .max_size(4)
        .build(ConnectionManager::<SqliteConnection>::new(
            &dotenv_database_url,
        ))
        .expect("failed to create db connection pool");

    let addr = String::from("0.0.0.0:8000"); // In docker container
                                             //    let addr = String::from("localhost:8080");

    let route: Vec<
        Route<
            fn(
                HttpRequest,
                String,
                Option<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
                Option<Tera>,
            ) -> Result<HttpResponse, HttpError>,
        >,
    > = vec![
        Route::new(
            "GET".to_string(),
            (String::from("/"), Some(String::from("/static/index.html"))),
            get_service as RouteService,
        )
        .unwrap(),
        Route::new(
            "GET".to_string(),
            (String::from("/static/post_example.html"), None),
            get_service as RouteService,
        )
        .unwrap(),
        Route::new(
            "GET".to_string(),
            (String::from("/form.html"), None),
            form as RouteService,
        )
        .unwrap(),
        Route::new(
            "POST".to_string(),
            (String::from("/form/memo"), None),
            memo_form as RouteService,
        )
        .unwrap(),
        Route::new(
            "DELETE".to_string(),
            (String::from("/form/memo"), None),
            memo_delete as RouteService,
        )
        .unwrap(),
        Route::new(
            "PUT".to_string(),
            (String::from("/form/memo"), None),
            memo_update as RouteService,
        )
        .unwrap(),
    ];
    let _dealer = WebDealer::new(&addr, route, (Some(db_pool), Some(templates))).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
