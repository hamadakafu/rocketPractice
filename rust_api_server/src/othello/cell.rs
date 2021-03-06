use std::fmt;
use std::ops::Add;

use serde_derive::{Deserialize, Serialize};

use {
    super::{
        error::OthelloError,
        LENGTH,
    },
    self::Direction::*,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct OthelloCell {
    pub point: Point,
    pub state: CellState,
}

impl OthelloCell {
    pub fn new(x: isize, y: isize) -> OthelloCell {
        OthelloCell {
            point: Point::new(x, y),
            state: CellState::Empty,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub enum CellState {
    White,
    Black,
    Empty,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::White => write!(f, "W"),
            CellState::Black => write!(f, "B"),
            CellState::Empty => write!(f, " "),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Deserialize, Serialize)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point { Point { x, y } }
    pub fn check_out_of_bounds(&self)
                               -> Result<(), OthelloError>
    {
        if let (true, _) | (_, true) = (
            self.x >= LENGTH || self.x < 0,
            self.y >= LENGTH || self.y < 0,
        ) { Err(OthelloError::OutOfBounds { point: *self }) } else { Ok(()) }
    }
    pub fn to_index(&self) -> usize { (self.x * LENGTH + self.y) as usize }
    pub fn neighbors(&self) -> Vec<Point> {
        Direction::change_to_vec().into_iter()
            .map(|dir| *self + dir)
            .filter(
                |point| point.check_out_of_bounds().is_ok()
            ).collect()
    }
}

impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self {
        match rhs {
            Up => {
                Point {
                    x: self.x.wrapping_sub(1),
                    y: self.y,
                }
            }
            UpRight => { self + Up + Right }
            Right => {
                Point {
                    x: self.x,
                    y: self.y.wrapping_add(1),
                }
            }
            DownRight => { self + Down + Right }
            Down => {
                Point {
                    x: self.x.wrapping_add(1),
                    y: self.y,
                }
            }
            DownLeft => { self + Down + Left }
            Left => {
                Point {
                    x: self.x,
                    y: self.y.wrapping_sub(1),
                }
            }
            UpLeft => { self + Up + Left }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    // clockwise
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn change_to_vec() -> Vec<Direction> {
        vec![
            Up,
            UpRight,
            Right,
            DownRight,
            Down,
            DownLeft,
            Left,
            UpLeft,
        ]
    }
}
