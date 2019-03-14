use std::fmt;

use serde_derive::{Deserialize, Serialize};

use self::othello_cell::{Point, OthelloCell, CellState, Direction};
use super::OthelloError;


#[cfg(test)]
mod tests;
pub mod othello_cell;

#[derive(Eq, PartialEq, Deserialize, Serialize)]
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

    fn change(&mut self, point: Point, cs: CellState) {
        self.0[point.to_index()].set_state(cs);
    }

    pub fn set(&mut self, point: Point, cs: CellState)
               -> Result<usize, OthelloError>
    {
        point.check_out_of_bounds()?;
        if cs == CellState::Empty { return Err(OthelloError::CantSetEmpty); }
        match self.get_cell(point).get_state() {
            CellState::Black | CellState::White =>
                Err(OthelloError::AlreadyOccupied {
                    cell: self.get_cell(point),
                }),
            CellState::Empty => {
                let n_changed_cell = self.reverse(point, cs)?;
                self.change(point, cs);
                Ok(n_changed_cell)
            }
        }
    }

    fn reverse(&mut self, point: Point, cs: CellState)
               -> Result<usize, OthelloError>
    {
        let changed_cell: usize = Direction::change_to_vec().into_iter()
            .filter_map(|dir| self.sandwich(dir, point + dir, cs))
            .sum();
        if changed_cell == 0 {
            return Err(
                OthelloError::CantSetAtCell { cell: self.get_cell(point) }
            );
        } else {
            Ok(changed_cell)
        }
    }

    fn sandwich(
        &mut self,
        dir: Direction,
        point: Point,
        cs: CellState,
    ) -> Option<usize>
    {
        if point.check_out_of_bounds().is_err() { return None; }

        let now_cell = self.get_cell(point);
        if now_cell.get_state() == CellState::Empty {
            None
        } else if now_cell.get_state() == cs {
            Some(0)
        } else {
            self.sandwich(dir, point + dir, cs).map(
                |n| {
                    self.change(point, cs);
                    n + 1
                }
            )
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
                write!(f, " {}", self.get_cell(Point::new(x, y)).get_state())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
