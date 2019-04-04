#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate mongodb;
#[macro_use(Model)]
extern crate wither_derive;

use rocket::{routes, get, catch, response::content};
use rocket_codegen::catchers;

mod othello;
mod api;

#[get("/")]
fn hello() -> String {
    String::from("hello\n")
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>", req.uri()))
}

fn main() {
    rocket::ignite().mount("/", routes![
        hello,
        api::create_room_handler,
        api::get_rooms_handler,
        api::set_piece_handler,
        api::delete_room_handler,
    ]).register(catchers![not_found]).launch();
}
