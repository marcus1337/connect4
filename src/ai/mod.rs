
pub mod evaluator;

use super::ai::evaluator::Evaluator;
use super::board::tile;
use super::board::Board;
use super::board::GameResult;
use super::board::line;
use super::board::tile::Brick;
use rayon::prelude::*;

fn get_possible_column_placements(board: &Board) -> Vec<usize> {
    let mut placements = Vec::new();
    for i in 0..7 {
        if board.can_place(i){
            placements.push(i as usize);
        }
    }
    placements
}

fn assign_minimax_score(maximizing_player: bool, score: i32, best_score: &mut i32, alpha: &mut i32, beta: &mut i32) {
    if maximizing_player {
        *best_score = std::cmp::max(*best_score, score);
        *alpha = std::cmp::max(*alpha, *best_score);
    } else {
        *best_score = std::cmp::min(*best_score, score);
        *beta = std::cmp::min(*beta, *best_score);
    }
}

fn max_minimax(board: Board, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    let mut best_score = std::i32::MIN;
    for col in get_possible_column_placements(&board) {
        let mut board_copy = board.clone();
        board_copy.place(col as i32);
        let score = minimax(board_copy.clone(), depth-1, false, alpha, beta);
        best_score = std::cmp::max(best_score, score);
        alpha = std::cmp::max(alpha, best_score);
        if beta <= alpha {
            return best_score;
        }
    }
    best_score
}

fn min_minimax(board: Board, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    let mut best_score = std::i32::MAX;
    for col in get_possible_column_placements(&board) {
        let mut board_copy = board.clone();
        board_copy.place(col as i32);
        let score = minimax(board_copy.clone(), depth-1, true, alpha, beta);
        best_score = std::cmp::min(best_score, score);
        beta = std::cmp::min(beta, best_score);
        if beta <= alpha {
            return best_score;
        }
    }
    best_score
}

fn minimax(board: Board, depth: i32, maximizing_player: bool, alpha: i32, beta: i32) -> i32 {
    if board.is_done() || depth == 0 {
        return Evaluator::new(board).get_score(); 
    }
    if maximizing_player{
        return max_minimax(board, depth, alpha, beta);
    }else{
        return min_minimax(board, depth, alpha, beta);
    }
}

pub fn get_column_placement(board: Board) -> i32 {
    let mut chosen_col: i32 = -1;
    let maximizing_player = board.get_next_brick() == Brick::One;
    let mut best_score = if maximizing_player { std::i32::MIN } else {std::i32::MAX};

    let possible_columns = get_possible_column_placements(&board);
    let scores: Vec<i32> = possible_columns.par_iter().map(|&col| {
        let mut board_copy = board.clone();
        board_copy.place(col as i32);
        minimax(board_copy, 7, !maximizing_player, std::i32::MIN, std::i32::MAX)
    }).collect();

    for (col, score) in possible_columns.iter().zip(scores.iter()) {
        if maximizing_player && score > &best_score {
            best_score = *score;
            chosen_col = *col as i32;
        }else if !maximizing_player && score < &best_score {
            best_score = *score;
            chosen_col = *col as i32;
        }
    }

    chosen_col
}


