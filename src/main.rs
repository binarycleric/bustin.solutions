#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use chrono::prelude::*;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    let dt = Local::now();

    context.insert("thursday", dt.weekday() == Weekday::Thu);
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
