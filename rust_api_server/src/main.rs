#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate mongodb;
#[macro_use(Model)]
extern crate wither_derive;
#[macro_use]
extern crate failure;

use rocket::{routes, get, catch, response::content};
use mongodb::{
    ThreadedClient,
    db::{Database, ThreadedDatabase},
    // coll::options::IndexModel,
    // oid::ObjectId,
};
use wither::prelude::Model;
use rocket_codegen::catchers;

mod othello;
mod api;

#[get("/")]
fn hello() -> String {
    String::from("hello\n")
}
#[get("/mongodb_test")]
fn mm() -> String {
    let db: Database = mongodb::Client::with_uri("mongodb://mongodb-service:27017/")
        .expect("Failed to initialize client.").db("myDB");
    let othellos = othello::Othello::find_one(db.clone(), None, None).expect("wtf");
    format!("othello: {:?}", othellos)
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>
            <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
                          req.uri()))
}

fn main() {
    rocket::ignite().mount("/", routes![
        hello,
        mm,
        api::create_room,
        api::get_rooms,
    ]).register(catchers![not_found]).launch();
}

