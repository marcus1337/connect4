
#[path = "../src/lib.rs"]
mod lib;
use lib::Connect4;

#[cfg(test)]
mod tests {    
    use super::Connect4;
    use super::lib::board::GameResult;
    use super::lib::board::Board;

    #[test]
    fn can_win_vertical() {
        let mut connect4 = Connect4::make();
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        let result = connect4.board.get_result();
        assert_eq!(result, GameResult::OneWin);
    }

    #[test]
    fn can_win_horizontal() {
        let mut connect4 = Connect4::make();
        connect4.board.place(6);
        connect4.board.place(0);
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(1);
        connect4.board.place(2);
        connect4.board.place(2);
        connect4.board.place(3);
        let result = connect4.board.get_result();
        assert_eq!(result, GameResult::TwoWin);
    }

    #[test]
    fn can_win_diagonally() {
        let mut connect4 = Connect4::make();
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(1);
        connect4.board.place(3);
        connect4.board.place(2);
        connect4.board.place(2);
        connect4.board.place(2);
        connect4.board.place(6);
        connect4.board.place(3);
        connect4.board.place(3);
        connect4.board.place(3);
        let result = connect4.board.get_result();
        assert_eq!(result, GameResult::OneWin);
    }

}