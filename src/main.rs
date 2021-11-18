use rserver::web_dealer::get_service;
use rserver::web_dealer::{Route, WebDealer};


#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate tera;
extern crate serde;

mod model;
mod schema;

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
    r2d2::{self, ConnectionManager}
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
//use std::str;
use tera::{Context, Tera};
use schema::memos;

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    content: String,
}

async fn form(
    pool: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, Error>
//where
//    C :'static + diesel::Connection<Backend = UsesAnsiSavepointSyntax>
{
    let mut ctx = Context::new();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let memos = memos::table
        .load::<crate::model::Memo>(&conn)
        .expect("Error loading cards");
    ctx.insert("memos", &memos);
    let view = tmpl
        .render("form.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

async fn memo_form(
    pool: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    params: web::Form<FormParams>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse, HttpError>
//where
//    C :'static + diesel::Connection<Backend = UsesAnsiSavepointSyntax>
{
    let new_memo = crate::model::NewMemo {
        content: String::from(&params.content),
    };
    let conn = pool.get().expect("couldn't get db connection from pool");
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
        .render("form.tera", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn main() {
    //    let addr = String::from("0.0.0.0:8000");      // In docker container
    let addr = String::from("localhost:8080");

    let route = vec![
        Route::new(
            "GET".to_string(),
            (String::from("/"), Some(String::from("/static/index.html"))),
            get_service,
        )
        .unwrap(),
        Route::new(
            "GET".to_string(),
            (String::from("/static/post_example.html"), None),
            get_service,
        )
        .unwrap(),
    ];
    let _dealer = WebDealer::new(&addr, route).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = Tera::new("templates/**/*").unwrap();

    let database_url = "DATABASE_URL".to_owned();

    dotenv().ok();
    let dotenv_database_url = std::env::var(database_url)
        .expect("DATABASE_URL must be set");
    let db_pool = r2d2::Pool::builder()
        .max_size(4)
        .build(ConnectionManager::<SqliteConnection>::new(&dotenv_database_url))
        .expect("failed to create db connection pool");
    
    HttpServer::new(move || {
        App::new()
            .data(templates.clone())
            .data(db_pool.clone())
            .route("/", web::get().to(greet))
            .route("/form", web::get().to(form))
            .route("/form/memo", web::post().to(memo_form))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await

    //    let addr = String::from("0.0.0.0:8000");      // In docker container
    let addr = String::from("localhost:8080"); 
    let _dealer = WebDealer::new(&addr).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}

