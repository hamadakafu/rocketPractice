use super::*;

#[test]
fn test_board_new() {
    let b = Board::new();
    assert_eq!(CellState::Black, b.get_cell(Point::new(3, 4)).get_state());
    assert_eq!(CellState::White, b.get_cell(Point::new(4, 4)).get_state());
}

#[test]
fn test_board_change() {
    let mut b = Board::new();
    let test_table: Vec<(Point, CellState)> = vec![
        (Point::new(3, 5), CellState::White),
        (Point::new(7, 7), CellState::Black),
    ];
    for test in test_table {
        b.change(test.0, test.1);
        assert_eq!(b.get_cell(test.0).get_state(), test.1);
    }
}

#[test]
fn test_board_set() {
    let test_table: Vec<(Point, CellState, Result<usize, OthelloError>)> = vec![
        (
            Point::new(10, 3), CellState::Black,
            Err(OthelloError::OutOfBounds { point: Point::new(10, 3) })
        ),
        (
            Point::new(3, 7), CellState::Empty,
            Err(OthelloError::CantSetEmpty)
        ),
        (
            Point::new(3, 5), CellState::White, Ok(1)
        ),
        (
            Point::new(7, 7), CellState::Black,
            Err(OthelloError::CantSetAtCell { cell: OthelloCell::new(7, 7) })
        ),
    ];
    let mut b = Board::new();
    for test in test_table {
        assert_eq!(b.set(test.0, test.1), test.2);
    }
}
