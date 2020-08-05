#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index() -> Template {

    let mut context = HashMap::new();
    context.insert("name","yoda");
    Template::render("index",&context)
}

fn main() {
    rocket::ignite()
	.attach(Template::fairing())
	.mount("/", routes![index]).launch();
}

