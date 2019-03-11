#![feature(proc_macro_hygiene, decl_macro)]

// Todo: write tests

use rocket::{routes, get};
mod othello;

#[get("/hello")]
fn hello() -> String {
    String::from("hello")
}

fn main() {
    // rocket::ignite().mount("/", routes![
    //     hello,
    // ]).launch();
}

