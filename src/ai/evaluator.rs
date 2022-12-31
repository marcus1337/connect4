use super::line::Line;
use super::tile::Brick;
use super::tile::Tile;
use super::Board;
use super::GameResult;

pub struct Evaluator {
    board: Board,
    player_brick: Brick,
    player_tile: Tile,
    lines: Vec<Line>,
}

impl Evaluator {
    pub fn new(board: Board) -> Self {
        let next_brick = board.get_next_brick();
        Self {
            board: board,
            player_tile: Tile::Brick(next_brick),
            player_brick: next_brick,
            lines: board.get_lines(),
        }
    }

    fn flip_evaluation_bricks(&mut self) {
        self.player_brick.flip();
        self.player_tile = Tile::Brick(self.player_brick);
    }

    fn is_line_3_left_ended(&self, line: &Line) -> bool {
        let line_3 = [self.player_tile; 3];
        if line.tiles[1..4] == line_3 && line.tiles[0] == Tile::Empty {
            return true;
        }
        if line.has_next_left_tile() {
            let next_left_tile = self.board.get_tile(line.get_next_left_point());
            return line.tiles[0..3] == line_3 && next_left_tile == self.player_tile;
        }
        false
    }

    fn is_line_3_right_ended(&self, line: &Line) -> bool {
        let line_3 = [self.player_tile; 3];
        if line.tiles[0..3] == line_3 && line.tiles[3] == Tile::Empty {
            return true;
        }
        if line.has_next_right_tile() {
            let next_right_tile = self.board.get_tile(line.get_next_right_point());
            return line.tiles[1..4] == line_3 && next_right_tile == self.player_tile;
        }
        false
    }

    fn count_lines_3_open_ended(&self) -> i32 {
        self.lines
            .iter()
            .filter(|&line| self.is_line_3_left_ended(line) && self.is_line_3_right_ended(line))
            .count() as i32
    }

    fn count_lines_3_single_ended(&self) -> i32 {
        self.lines
            .iter()
            .filter(|&line| self.is_line_3_left_ended(line) ^ self.is_line_3_right_ended(line))
            .count() as i32
    }

    fn count_lines_2_open_ended(&self) -> i32 {
        let line_2 = [self.player_tile; 2];
        let empty_line_2 = [Tile::Empty; 2];
        let mut counter = 0;
        for line in self.lines.iter() {
            if line.tiles[0..2] == line_2 && line.tiles[2..4] == empty_line_2 {
                counter += 1;
            }
            if line.tiles[2..4] == line_2 && line.tiles[0..2] == empty_line_2 {
                counter += 1;
            }
            if line.tiles[1..3] == line_2 && line.tiles[0] == Tile::Empty && line.tiles[3] == Tile::Empty {
                counter += 1;
            }
        }
        counter
    }

    fn get_game_end_score(&self) -> i32 {
        let game_result = self.board.get_result();
        if game_result == GameResult::Draw {
            return 0;
        } else if game_result == GameResult::OneWin && self.player_brick == Brick::One {
            return 100;
        } else {
            return -100;
        }
    }

    fn get_player_score(&self) -> i32 {
        let num_lines_3_open_ended = self.count_lines_3_open_ended();
        let num_lines_3_single_ended = self.count_lines_3_single_ended();
        let num_lines_2_open_ended = self.count_lines_2_open_ended();
        num_lines_3_open_ended * 10 + num_lines_3_single_ended * 5 + num_lines_2_open_ended
    }

    pub fn get_score(&mut self) -> i32 {
        if self.board.is_done() {
            return self.get_game_end_score();
        }
        let score = self.get_player_score();
        self.flip_evaluation_bricks();
        let score_reduction = self.get_player_score();
        self.flip_evaluation_bricks();
        score - score_reduction
    }
}
