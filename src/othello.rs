#[cfg(test)]
mod tests;
mod othello_board;

use std::fmt;

use failure::Fail;

use crate::othello::othello_board::{
    Board,
    othello_cell::{OthelloCell, CellState, Point},
};

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
}


#[derive()]
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
    board: Board,
}

impl fmt::Debug for Othello {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "othello");
        writeln!(f, "{:?}", self.board)
    }
}

const LENGTH: isize = 8;

impl Othello {
    pub fn new() -> Othello {
        Othello {
            board: Board::new()
        }
    }


    pub fn set(&mut self, x: isize, y: isize, piece: char)
               -> Result<(), OthelloError>
    {
        match &piece {
            'W' => self.board.set(Point::new(x, y), CellState::White),
            'B' => self.board.set(Point::new(x, y), CellState::Black),
            _ => Err(OthelloError::NoExistChar { c: piece })
        }
    }
}

