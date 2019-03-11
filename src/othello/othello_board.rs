pub mod othello_cell;

use std::fmt;

use self::othello_cell::{Point, OthelloCell, CellState, Direction};
use super::OthelloError;

pub struct Board(Vec<OthelloCell>);

impl Board {
    pub fn new() -> Board {
        Board(
            (0..super::LENGTH).flat_map(
                |x| (0..super::LENGTH).map(
                    move |y| OthelloCell::new(x, y)
                )
            ).collect()
        )
    }
    fn change(&mut self, point: Point, cs: CellState)
              -> Result<(), OthelloError>
    {
        point.check_out_of_bounds()?;
        self.0[point.to_index()].set(cs);
        Ok(())
    }
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
            Empty => {
                self.change(point, cs);
                self.reverse(point, cs)
            }
        }
    }
    fn reverse(&mut self, point: Point, cs: CellState)
               -> Result<(), OthelloError>
    {
        point.check_out_of_bounds()?;
        unimplemented!()
    }
    fn reverse_helper(
        &mut self,
        dir: Direction,
        x: isize,
        y: isize,
        cs: CellState,
    )
        -> Result<(), OthelloError>
    {
        unimplemented!()
    }

}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " | 0 1 2 3 4 5 6 7")?;
        writeln!(f, "------------------")?;
        for x in 0..super::LENGTH {
            write!(f, "{}|", x)?;
            for y in 0..super::LENGTH {
                write!(f, " {:?}", self.0[(x * super::LENGTH + y) as usize].get_state())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
