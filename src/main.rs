#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate mongodb;
#[macro_use(Model)]
extern crate wither_derive;

use rocket::{routes, get};
use mongodb::{
    ThreadedClient,
    // db::{Database, ThreadedDatabase},
    // coll::options::IndexModel,
    // oid::ObjectId,
};
use wither::prelude::Model;

mod othello;


#[get("/hello")]
fn hello() -> String {
    String::from("hello")
}

fn main() -> Result<(), othello::OthelloError>{
    let db = mongodb::Client::with_uri("mongodb://localhost:27017/").unwrap().db("myDB");
    let mut othello_game = othello::Othello::new(String::from("hoge"));
    othello_game.save(db.clone(), None).unwrap();
    // othello_game.start()
    // rocket::ignite().mount("/", routes![
    //     hello,
    // ]).launch();
    Ok(())
}

