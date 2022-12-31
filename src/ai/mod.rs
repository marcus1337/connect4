
pub mod evaluator;

use super::board::tile;
use super::board::Board;
use super::board::GameResult;
use super::board::line;

fn get_possible_column_placements(board: &Board) -> Vec<usize> {
    let mut placements = Vec::new();
    for i in 0..7 {
        if board.can_place(i){
            placements.push(i as usize);
        }
    }
    placements
}

fn get_column_placement(board: Board) -> i32 {

    for placement in get_possible_column_placements(&board){

    }

    0
}


