
#[path = "../src/lib.rs"]
mod lib;
use lib::Connect4;

#[cfg(test)]
mod tests {    
    use super::Connect4;
    use super::lib::board::GameResult;
    use super::lib::board::tile::Point;

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
    fn can_get_win_line() {
        let mut connect4 = Connect4::make();
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        connect4.board.place(1);
        connect4.board.place(0);
        let line = connect4.board.get_win_line();
        assert_eq!(line.points, [Point(0,0), Point(0,1), Point(0,2), Point(0,3)]);
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