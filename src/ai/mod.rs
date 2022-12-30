
use super::board::Board;
use super::board::GameResult;

fn is_win_col(mut board: Board, col: i32) -> bool {
    if !board.can_place(col) {
        return false;
    }
    board.place(col);
    let game_result = board.get_result();
    game_result == GameResult::OneWin || game_result == GameResult::TwoWin
}
