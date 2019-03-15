#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate mongodb;
#[macro_use(Model)]
extern crate wither_derive;

use rocket::{routes, get};
use mongodb::{
    ThreadedClient,
    db::{Database, ThreadedDatabase},
    // coll::options::IndexModel,
    // oid::ObjectId,
};
use wither::prelude::Model;

mod othello;


#[get("/hello")]
fn hello() -> String {
    String::from("hello unko unko\n")
}
#[get("/mongodb_test")]
fn mm() -> String {
    let db: Database = mongodb::Client::with_uri("mongodb://mongodb-service:27017/")
        .expect("Failed to initialize client.").db("myDB");
    let othellos = othello::Othello::find_one(db.clone(), None, None).expect("wtf");
    format!("othello: {:?}", othellos)
}

fn main() {
    rocket::ignite().mount("/", routes![
        hello,
        mm,
    ]).launch();
}

