#[cfg(test)]
mod tests;

use std::fmt;
use std::ops::Add;

use super::super::OthelloError;
use self::Direction::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct OthelloCell {
    point: Point,
    state: CellState,
}

impl OthelloCell {
    pub fn new(x: isize, y: isize) -> OthelloCell {
        OthelloCell {
            point: Point::new(x, y),
            state: CellState::Empty,
        }
    }
    pub fn set_state(&mut self, cs: CellState) { self.state = cs; }
    pub fn get_state(&self) -> CellState { self.state }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum CellState {
    White,
    Black,
    Empty,
}

impl fmt::Debug for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CellState::White => write!(f, "W"),
            CellState::Black => write!(f, "B"),
            CellState::Empty => write!(f, "E"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
            self.x >= super::super::LENGTH || self.x < 0,
            self.y >= super::super::LENGTH || self.y < 0,
        ) { Err(OthelloError::OutOfBounds { point: *self }) } else { Ok(()) }
    }
    pub fn to_index(&self) -> usize { (self.x * super::super::LENGTH + self.y) as usize }
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
    pub fn to_vec() -> Vec<Direction> {
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
