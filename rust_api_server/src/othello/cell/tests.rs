use super::*;
use crate::othello::OthelloError::OutOfBounds;

#[test]
fn test_othello_cell() {
    let mut oc: OthelloCell = OthelloCell::new(3, 4);
    assert_eq!(oc.state, CellState::Empty);
    oc.state = CellState::White;
    assert_eq!(oc.state, CellState::White);
}

#[test]
fn test_point_add() {
    let mut p: Point = Point::new(3, 4);
    assert_eq!(Point::new(2, 4), p + Direction::Up);
    assert_eq!(Point::new(2, 5), p + Direction::UpRight);
    assert_eq!(Point::new(3, 3), p + Direction::Left);
    assert_eq!(Point::new(4, 3), p + Direction::DownLeft);
}

#[test]
fn test_point_check_out_of_bounds() {
    let test_table: Vec<(Result<(), super::super::OthelloError>, Point)> = vec![
        (Err(OutOfBounds { point: Point::new(3, 10) }), Point::new(3, 10)),
        (Err(OutOfBounds { point: Point::new(11, 0) }), Point::new(11, 0)),
        (Err(OutOfBounds { point: Point::new(-3, 4) }), Point::new(-3, 4)),
        (Err(OutOfBounds { point: Point::new(4, -10) }), Point::new(4, -10)),
    ];
    for test in test_table {
        assert_eq!(test.0, test.1.check_out_of_bounds());
    }
}

#[test]
fn test_point_to_index() {
    let test_table: Vec<(usize, Point)> = vec![
        (3, Point::new(0, 3)),
        (54, Point::new(6, 6)),
    ];
    for test in test_table {
        assert_eq!(test.0, test.1.to_index());
    }
}
