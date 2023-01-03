use super::line::Line;
use super::tile::Brick;
use super::tile::Tile;
use super::Board;
use super::GameResult;

pub struct Evaluator {
    board: Board,
    lines: Vec<Line>,
}

impl Evaluator {
    pub fn new(board: Board) -> Self {
        Self {
            board: board,
            lines: board.get_lines(),
        }
    }

    fn is_line_3_left_ended(&self, line: &Line, player_tile: Tile) -> bool {
        let line_3 = [player_tile; 3];
        if line.tiles[1..4] == line_3 && line.tiles[0] == Tile::Empty {
            return true;
        }
        if line.has_next_left_tile() {
            let next_left_tile = self.board.get_tile(line.get_next_left_point());
            return line.tiles[0..3] == line_3 && next_left_tile == player_tile;
        }
        false
    }

    fn is_line_3_right_ended(&self, line: &Line, player_tile: Tile) -> bool {
        let line_3 = [player_tile; 3];
        if line.tiles[0..3] == line_3 && line.tiles[3] == Tile::Empty {
            return true;
        }
        if line.has_next_right_tile() {
            let next_right_tile = self.board.get_tile(line.get_next_right_point());
            return line.tiles[1..4] == line_3 && next_right_tile == player_tile;
        }
        false
    }

    fn count_lines_3_open_ended(&self, player_tile: Tile) -> i32 {
        self.lines
            .iter()
            .filter(|&line| self.is_line_3_left_ended(line, player_tile) && self.is_line_3_right_ended(line, player_tile))
            .count() as i32
    }

    fn count_lines_3_single_ended(&self, player_tile: Tile) -> i32 {
        self.lines
            .iter()
            .filter(|&line| self.is_line_3_left_ended(line, player_tile) ^ self.is_line_3_right_ended(line, player_tile))
            .count() as i32
    }

    /*fn count_lines_2_open_ended(&self) -> i32 {
        let line_2 = [self.player_tile; 2];
        let mut counter = 0;
        for line in self.lines.iter() {
            if line.tiles[1..3] == line_2 && line.tiles[0] == Tile::Empty && line.tiles[3] == Tile::Empty {
                counter += 2;
            }
        }
        counter
    }*/

    fn get_game_end_score(&self) -> i32 {
        let game_result = self.board.get_result();
        if game_result == GameResult::Draw {
            return 0;
        } else if game_result == GameResult::OneWin {
            return 1000;
        } else {
            return -1000;
        }
    }

    fn get_player_score(&self, player_tile: Tile) -> i32 {
        let num_lines_3_open_ended = self.count_lines_3_open_ended(player_tile);
        let num_lines_3_single_ended = self.count_lines_3_single_ended(player_tile);
        num_lines_3_open_ended * 5 + num_lines_3_single_ended
    }

    pub fn get_score(&self) -> i32 {
        if self.board.is_done() {
            return self.get_game_end_score();
        }
        let score = self.get_player_score(Tile::Brick(Brick::One));
        let score_reduction = self.get_player_score(Tile::Brick(Brick::Two));
        score - score_reduction
    }
}
