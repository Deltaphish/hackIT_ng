#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

pub mod db;

use crate::db::{Record};

#[database("hackit")]
struct UserRecordsConn(diesel::PgConnection);

#[get("/records")]
fn records( conn : UserRecordsConn ) -> Template {

    let recs = Record::all(&conn).unwrap();
    
    let mut context = HashMap::new();
    context.insert("records",recs);
    Template::render("records",&context)
}

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("name","yoda");
    Template::render("index",&context)
}

fn main() {
    rocket::ignite()
	.attach(Template::fairing())
	.attach(UserRecordsConn::fairing())
	.mount("/", routes![index,records]).launch();
}

