use rocket::{get, Data};
use mongodb::{
    ThreadedClient,
    db::Database,
};
use super::othello;
use failure::{Fail, Error};
use wither::prelude::*;
use rocket_contrib::json;
use rocket::response::content;

use super::othello::Othello;
use crate::api::error::ApiError;

mod error;
#[cfg(test)]
mod tests;

fn connect_db() -> Result<Database, ApiError> {
    Ok(mongodb::Client::with_uri("mongodb://mongodb-service:27017/")?.db("myDB"))
}

#[get("/othello/create/<room>")]
pub fn create_room(room: String) -> Result<json::JsonValue, ApiError> {
    create_room_helper(&room)
        .map(|()| json!({ "status": true }))
}


fn create_room_helper(room_name: &String) -> Result<(), ApiError> {
    let db: Database = connect_db()?;
    // search room
    match Othello::find_one(db.clone(), Some(doc! {
            "room_name": room_name,
       }), None).map_err(|e| ApiError::MongoError(e))? {
        Some(_othello_game) => Err(ApiError::AlreadyExistRoom(room_name.to_string())),
        None => {
            Othello::new(room_name).save(db.clone(), None)?;
            Ok(())
        }
    }
}

#[get("/othello/rooms")]
pub fn get_rooms() -> Result<json::Json<Vec<String>>, ApiError> {
    let rooms: Vec<String> = get_rooms_helper()?;
    Ok(json::Json(rooms))
}

fn get_rooms_helper() -> Result<Vec<String>, ApiError> {
    let db: Database = connect_db()?;
    match Othello::find(db.clone(), None, None) {
        Ok(games) => Ok(games.into_iter()
            .map(|othello| othello.room_name).collect()),
        Err(_) => Ok(vec![]) // This error imply empty.
    }
}
