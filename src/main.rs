#![feature(proc_macro_hygiene, decl_macro, result_map_or_else)]

use rocket::{routes, get};

mod othello;


#[get("/hello")]
fn hello() -> String {
    String::from("hello")
}

fn main() -> Result<(), othello::OthelloError>{
    let othello_game = othello::Othello::new();
    othello_game.start()
    // rocket::ignite().mount("/", routes![
    //     hello,
    // ]).launch();
}

