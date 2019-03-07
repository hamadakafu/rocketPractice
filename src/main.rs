#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{routes, get};

#[get("/hello")]
fn hello() -> String {
    String::from("hello")
}

fn main() {
    rocket::ignite().mount("/", routes![
        hello,
    ]).launch();
}
