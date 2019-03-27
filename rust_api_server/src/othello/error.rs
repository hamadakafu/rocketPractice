use failure::Fail;

use super::cell::{Point, OthelloCell};

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

