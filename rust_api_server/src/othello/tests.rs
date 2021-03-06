use super::*;

#[test]
fn test_othello_set() {
    let mut o = Othello::new("hoge");
    o.set(3, 5, 'W');
    let mut o_hat = Othello::new("hoge");
    let mut o_hat_board = Board::new();
    o_hat_board.set(Point::new(3, 5), CellState::White).unwrap();
    o_hat.board = o_hat_board;
    assert_eq!(o, o_hat);
}