#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use chrono::prelude::*;
use std::env;


#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    let dt = Local::now();
    let force_bustin = env::var("FORCE_BUSTIN").unwrap_or("0".to_string()) == "1";

    if force_bustin || dt.weekday() == Weekday::Thu {
        context.insert("bustin", true);
    } else {
        context.insert("bustin", false);
    }
    context.insert("forced", force_bustin);

    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
