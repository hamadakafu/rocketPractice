use std::fmt;

use failure::Fail;

use serde_derive::{Deserialize, Serialize};
use mongodb::oid::ObjectId;
use mongodb::coll::options::IndexModel;

use crate::othello::othello_board::{
    Board,
    othello_cell::{
        OthelloCell,
        CellState,
        Point
    },
};

#[cfg(test)]
mod tests;
mod othello_board;
#[macro_use]
mod io;

#[derive(Debug, Fail, Eq, PartialEq)]
pub enum OthelloError {
    #[fail(display = "Out of bounds => point: {:?}", point)]
    OutOfBounds {
        point: Point,
    },
    #[fail(display = "This Cell is already occupied => cell state: {:?}", cell)]
    AlreadyOccupied {
        cell: OthelloCell,
    },
    #[fail(display = "Can't set Empty")]
    CantSetEmpty,
    #[fail(display = "Don't exist char => char: {}", c)]
    NoExistChar {
        c: char,
    },
    #[fail(display = "Can't set at this cell: cell: {:?}", cell)]
    CantSetAtCell {
        cell: OthelloCell,
    },
}


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
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    id: Option<ObjectId>,
    #[model(index(index="dsc", unique="true"))]
    room_name: String,
    #[model(index(index="dsc"))]
    board: Board,
    #[model(index(index="dsc"))]
    n_turn: isize,
    #[model(index(index="dsc"))]
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
    pub fn new(room_name: String) -> Othello {
        Othello {
            id: None,
            room_name,
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
            let set_result: Result<usize, OthelloError> = match self.next_turn {
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
    fn next(&mut self) {
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

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
enum OthelloPlayer {
    White,
    Black,
}
