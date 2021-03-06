#[cfg(test)]
mod tests;
mod board;
#[macro_use]
mod io;
pub mod error;
mod cell;

use std::fmt;

use serde_derive::{Deserialize, Serialize};
use mongodb::oid::ObjectId;
use mongodb::coll::options::IndexModel;

use crate::othello::{
    board::Board,
    cell::{
        CellState,
        Point,
    },
    error::OthelloError,
};

#[derive(Model, Eq, PartialEq, Deserialize, Serialize)]
pub struct Othello {
    /*
     \ y0 - y1 - .. - yN
    x0 o    o    ..   o
    |
    x1 o    o    ..   o
    |
    :  :    :    ::   :
    |
    xN o    o    ..   o
    */
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[model(index(index = "dsc", unique = "true"))]
    pub room_name: String,
    #[model(index(index = "dsc"))]
    board: Board,
    #[model(index(index = "dsc"))]
    n_turn: isize,
    #[model(index(index = "dsc"))]
    next_turn: OthelloPlayer,
}

impl fmt::Debug for Othello {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "othello")?;
        writeln!(f, "{:?}", self.board)
    }
}

impl fmt::Display for Othello {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "room_name: {}", self.room_name)?;
        writeln!(f, "turn: {}, player: {:?}", self.n_turn, self.next_turn)?;
        writeln!(f, "{:?}", self.board)
    }
}

const LENGTH: isize = 8;

impl Othello {
    pub fn new(room_name: &str) -> Othello {
        Othello {
            id: None,
            room_name: String::from(room_name),
            board: Board::new(),
            n_turn: 0,
            next_turn: OthelloPlayer::White,
        }
    }
    pub fn start(mut self) -> Result<(), OthelloError> {
        let handle = std::io::stdin();
        let mut r = io::StdinReader::new(handle.lock());
        while self.n_turn < LENGTH * LENGTH - 4 {
            println!("{}", self);
            println!("input x y (ex: 3 4): ");
            let (x, y): (isize, isize) = input!(r, isize, isize);
            let set_result: Result<usize, OthelloError> =
                match self.next_turn {
                    OthelloPlayer::White => {
                        self.set(x, y, 'W')
                    }
                    OthelloPlayer::Black => {
                        self.set(x, y, 'B')
                    }
                };
            match set_result {
                Err(OthelloError::OutOfBounds { point: _ }) |
                Err(OthelloError::CantSetAtCell { cell: _ }) |
                Err(OthelloError::AlreadyOccupied { cell: _ }) => {
                    println!("Invalid input.");
                    println!("Could you input again?");
                    continue;
                }
                e => {
                    e?;
                }
            }
            self.next();
        }
        Ok(())
    }

    /// change turn
    /// # Example
    /// ```
    /// let mut o = Othello::new();
    /// o.next();
    /// ```
    pub fn next(&mut self) {
        match &self.next_turn {
            OthelloPlayer::White => {
                self.next_turn = OthelloPlayer::Black;
            }
            OthelloPlayer::Black => {
                self.next_turn = OthelloPlayer::White;
            }
        }
        self.n_turn += 1;
    }

    pub fn set(&mut self, x: isize, y: isize, piece: char)
               -> Result<usize, OthelloError>
    {
        match &piece {
            'W' => self.board.set(Point::new(x, y), CellState::White),
            'B' => self.board.set(Point::new(x, y), CellState::Black),
            _ => Err(OthelloError::NoExistChar { c: piece })
        }
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Copy, Clone)]
enum OthelloPlayer {
    White,
    Black,
}
