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
use crate::othello::error::OthelloError;

mod error;
#[cfg(test)]
mod tests;

fn connect_db() -> Result<Database, APIError> {
    Ok(mongodb::Client::with_uri("mongodb://mongodb-service:27017/")
        .map_err(APIErrorKind::from)?.db("myDB"))
}

#[get("/othello/create/room/<room_name>")]
pub fn create_room_handler(room_name: String) -> Result<json::JsonValue, Status> {
    create_room_helper(&room_name)
        .map(|()| json!(true))
        .map_err(|e| e.to_status())
}

fn create_room_helper(room_name: &String) -> Result<(), APIError> {
    let db: Database = connect_db()?;
    // search room
    let maybe_room = Othello::find_one(
        db.clone(),
        Some(doc! {"room_name": room_name}), None,
    ).map_err(APIErrorKind::from)?;
    match maybe_room {
        Some(_othello_game) => {
            Err(APIErrorKind::AlreadyExistRoom(room_name.to_string()).into())
        }
        None => {
            Othello::new(room_name).save(db.clone(), None)
                .map_err(APIErrorKind::from)?;
            Ok(())
        }
    }
}

#[get("/othello/delete/room/<room_name>")]
pub fn delete_room_handler(room_name: String) -> Result<json::JsonValue, Status> {
    delete_room(&room_name).map_err(|e| e.to_status())?;
    Ok(json!(true))
}

fn delete_room(room_name: &str) -> Result<(), APIError> {
    let db: Database = connect_db()?;
    let a = Othello::find_one_and_delete(db.clone(), doc! {"room_name": room_name}, None)
        .map_err(|e| APIError::from(APIErrorKind::from(e)))?;
    println!("{:?}", a);
    Ok(())
}


#[get("/othello/get/room")]
pub fn get_rooms_handler() -> Result<json::Json<Vec<String>>, Status> {
    let rooms: Vec<String> = get_rooms().map_err(|e| e.to_status())?;
    Ok(json::Json(rooms))
}


fn get_rooms() -> Result<Vec<String>, APIError> {
    let db: Database = connect_db()?;
    match Othello::find(db.clone(), None, None) {
        Ok(games) => Ok(games.into_iter()
            .map(|othello| othello.room_name).collect()),
        Err(_) => Ok(vec![]) // This error imply empty.
    }
}

#[get("/othello/set/<room_name>/<color>/<x>/<y>")]
pub fn set_piece_handler(room_name: String, color: String, x: isize, y: isize)
                         -> Result<json::JsonValue, Status> {
    let mut othello: Othello = get_one_game(&room_name).map_err(|e| e.to_status())?;
    if color == "white".to_owned() {
        othello.set(x, y, 'W')
            .map_err(|e| APIError::from(APIErrorKind::from(e)))
            .map_err(|e| e.to_status())?;
    } else if color == "black".to_owned() {
        othello.set(x, y, 'B')
            .map_err(|e| APIError::from(APIErrorKind::from(e)))
            .map_err(|e| e.to_status())?;
    } else {
        Err(Status::BadRequest)?;
    }
    othello.next();
    let othello = update_game(othello)
        .map_err(|e| e.to_status())?;
    Ok(json!(othello))
}

fn get_one_game(room_name: &str) -> Result<Othello, APIError> {
    let db = connect_db()?;
    let maybe_othello: Option<Othello> = Othello::find_one(db.clone(), Some(doc! {"room_name": room_name}), None)
        .map_err(|e| APIError::from(APIErrorKind::from(e)))?;
    if let Some(othello) = maybe_othello {
        Ok(othello)
    } else {
        Err(APIErrorKind::NotExistRoom(room_name.to_owned())).map_err(APIError::from)
    }
}

fn update_game(mut othello: Othello) -> Result<Othello, APIError> {
    delete_room(&othello.room_name)?;
    // othello data を save する
    let db: Database = connect_db()?;
    othello.save(db.clone(), None)
        .map_err(|e| APIError::from(APIErrorKind::from(e)))?;
    Ok(othello)
}

