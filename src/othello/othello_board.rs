#[cfg(test)]
mod tests;

pub mod othello_cell;

use std::fmt;

use self::othello_cell::{Point, OthelloCell, CellState, Direction};
use super::OthelloError;

#[derive(Eq, PartialEq)]
pub struct Board(pub Vec<OthelloCell>);

impl Board {
    pub fn new() -> Board {
        let mut b = Board(
            (0..super::LENGTH).flat_map(
                |x| (0..super::LENGTH).map(
                    move |y| OthelloCell::new(x, y)
                )
            ).collect()
        );
        b.change(Point::new(3, 3), CellState::White);
        b.change(Point::new(4, 3), CellState::Black);
        b.change(Point::new(3, 4), CellState::Black);
        b.change(Point::new(4, 4), CellState::White);
        b
    }

    fn get_cell(&self, point: Point) -> OthelloCell { self.0[point.to_index()] }

    fn change(&mut self, point: Point, cs: CellState)
    { self.0[point.to_index()].set_state(cs); }

    pub fn set(&mut self, point: Point, cs: CellState)
               -> Result<(), OthelloError>
    {
        point.check_out_of_bounds()?;
        if cs == CellState::Empty {
            return Err(OthelloError::CantSetEmpty);
        }
        match self.0[point.to_index()].get_state() {
            CellState::Black | CellState::White => Err(OthelloError::AlreadyOccupied {
                cell: self.0[point.to_index()]
            }),
            CellState::Empty => {
                let can_set = Direction::to_vec().into_iter().any(
                    |dir| {
                        let neighbor = point + dir;
                        neighbor.check_out_of_bounds().map_or_else(
                            |_| false,
                            |_| self.get_cell(neighbor).get_state() != CellState::Empty,
                        )
                    }
                );
                if can_set {
                    self.reverse(point, cs)
                } else {
                    Err(OthelloError::CantSetAtCell {
                        cell: self.get_cell(point)
                    })
                }
            }
        }
    }

    fn reverse(&mut self, point: Point, cs: CellState)
               -> Result<(), OthelloError>
    {
        point.check_out_of_bounds()?;
        self.change(point, cs);
        for dir in Direction::to_vec() {
            self.sandwich(dir, point + dir, cs);
        }
        Ok(())
    }

    fn sandwich(
        &mut self,
        dir: Direction,
        point: Point,
        cs: CellState,
    ) -> Result<bool, OthelloError>
    {
        if let Err(_) = point.check_out_of_bounds() { return Ok(false); }
        let now_cell = self.get_cell(point);
        if now_cell.get_state() == CellState::Empty {
            Ok(false)
        } else if now_cell.get_state() == cs {
            Ok(true)
        } else {
            match self.sandwich(dir, point + dir, cs) {
                Ok(do_reverse) if do_reverse => {
                    self.change(point, cs);
                    Ok(true)
                }
                e @ Err(_) => e,
                _ => Ok(false)
            }
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " | 0 1 2 3 4 5 6 7")?;
        writeln!(f, "------------------")?;
        for x in 0..super::LENGTH {
            write!(f, "{}|", x)?;
            for y in 0..super::LENGTH {
                write!(f, " {:?}", self.0[Point::new(x, y).to_index()].get_state())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
