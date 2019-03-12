#![feature(proc_macro_hygiene, decl_macro, result_map_or_else)]

use rocket::{routes, get};

mod othello;

#[get("/hello")]
fn hello() -> String {
    String::from("hello")
}

fn main() -> Result<(), othello::OthelloError>{
    let mut othello_game = othello::Othello::new();
    println!("{:?}", othello_game);
    othello_game.set(3, 5, 'W')?;
    println!("{:?}", othello_game);
    othello_game.set(2, 3, 'B')?;
    println!("{:?}", othello_game);
    // rocket::ignite().mount("/", routes![
    //     hello,
    // ]).launch();
    Ok(())
}

