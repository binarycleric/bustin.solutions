#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate chrono_tz;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use chrono::prelude::*;
use chrono_tz::America::New_York;

use std::env;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    let dt = Local::now().with_timezone(&New_York);
    let force_bustin = env::var("FORCE_BUSTIN").unwrap_or("0".to_string()) == "1";
    let bustin_time = force_bustin || dt.weekday() == Weekday::Thu;

    context.insert("bustin", bustin_time);
    context.insert("forced", force_bustin);

    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
