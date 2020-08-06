#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use serde::Serialize;
use rocket::State;
use rocket_contrib::templates::Template;

pub mod db;
pub mod challenge;

use crate::challenge::{Challenges,load_challenges};
use crate::db::{Record};

#[database("hackit")]
struct UserRecordsConn(diesel::PgConnection);

#[get("/records")]
fn records( conn : UserRecordsConn ) -> Template {

    let recs = Record::all(&conn).unwrap();

    #[derive(Serialize)]
    struct Context{
	records : Vec<Record>,
    }
    
    let ctx = Context{ records: recs };
    
    Template::render("records",&ctx)
}

#[get("/challenges")]
fn challenges(chs : State<ConstState>) -> Template {

    #[derive(Serialize)]
    struct Context<'a> {
	names : Vec<&'a String>,
    }
    
    let ctx = Context { names : chs.challenges.keys().collect() };
    Template::render("challenges",&ctx)
}

#[get("/")]
fn index() -> Template {

    #[derive(Serialize)]
    struct Context {
	name : &'static str,
    }
    let ctx = Context{ name : "Yoda" };
    
    Template::render("index",&ctx)
}

struct ConstState{
    challenges : Challenges,
}

fn main() {
    rocket::ignite()
	.attach(Template::fairing())
	.attach(UserRecordsConn::fairing())
	.manage(ConstState{ challenges : load_challenges("test_challenges") })
	.mount("/", routes![index,records,challenges]).launch();
}

