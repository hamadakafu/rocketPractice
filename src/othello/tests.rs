use super::*;

#[test]
fn test_othello_set() {
    let mut o = Othello::new("hoge".to_string());
    o.set(3, 5, 'W');
    let mut o_hat = Othello::new("hoge".to_string());
    let mut o_hat_board = Board::new();
    o_hat_board.set(Point::new(3, 5), CellState::White);
    o_hat.board = o_hat_board;
    assert_eq!(o, o_hat);
}