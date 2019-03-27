use rocket::{get, Data};
use mongodb::{
    ThreadedClient,
    db::Database,
};
use wither::prelude::Model;
use rocket_contrib::json;
use rocket::{http::Status};

use super::othello::Othello;
use crate::api::error::{APIError, APIErrorKind};

mod error;
#[cfg(test)]
mod tests;

fn connect_db() -> Result<Database, APIError> {
    Ok(mongodb::Client::with_uri("mongodb://mongodb-service:27017/")
        .map_err(APIErrorKind::from)?.db("myDB"))
}

#[get("/othello/create/room/<room_name>")]
pub fn create_room(room_name: String) -> Result<json::JsonValue, Status> {
    create_room_helper(&room_name)
        .map(|()| json!({ "status": true }))
        .map_err(|e| e.to_status())
}

fn create_room_helper(room_name: &String) -> Result<(), APIError> {
    let db: Database = connect_db()?;
    // search room
    match Othello::find_one(db.clone(), Some(doc! {
            "room_name": room_name,
       }), None).map_err(APIErrorKind::from)? {
        Some(_othello_game) => Err(APIErrorKind::AlreadyExistRoom(room_name.to_string()).into()),
        None => {
            Othello::new(room_name).save(db.clone(), None).map_err(APIErrorKind::from)?;
            Ok(())
        }
    }
}

#[get("/othello/get/room")]
pub fn get_rooms() -> Result<json::Json<Vec<String>>, Status> {
    let rooms: Vec<String> = get_rooms_helper().map_err(|e| e.to_status())?;
    Ok(json::Json(rooms))
}

fn get_rooms_helper() -> Result<Vec<String>, APIError> {
    let db: Database = connect_db()?;
    match Othello::find(db.clone(), None, None) {
        Ok(games) => Ok(games.into_iter()
            .map(|othello| othello.room_name).collect()),
        Err(_) => Ok(vec![]) // This error imply empty.
    }
}
