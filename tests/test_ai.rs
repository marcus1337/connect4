
#[path = "../src/lib.rs"]
mod lib;
use lib::Connect4;
use lib::ai::evaluator::Evaluator;
use lib::ai;

#[test]
fn can_evaluate() {
    let mut connect4 = Connect4::make();
    let player_brick = connect4.board.get_next_brick();
    connect4.board.place(0);
    connect4.board.place(1);
    connect4.board.place(0);
    connect4.board.place(1);
    connect4.board.place(0);
    connect4.board.place(1);
    let mut scorer = Evaluator::new(connect4.board, player_brick);
    let score = scorer.get_score();
    assert_eq!(0, score);
}

#[test]
fn can_evaluate_win() {
    let mut connect4 = Connect4::make();
    let player_brick = connect4.board.get_next_brick();
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    let mut scorer = Evaluator::new(connect4.board, player_brick);
    let score = scorer.get_score();
    assert_eq!(100, score);
}

#[test]
fn can_evaluate_loss() {
    let mut connect4 = Connect4::make();
    let mut player_brick = connect4.board.get_next_brick();
    player_brick.flip();
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    connect4.board.place(4);
    connect4.board.place(3);
    let mut scorer = Evaluator::new(connect4.board, player_brick);
    let score = scorer.get_score();
    assert_eq!(-100, score);
}


#[test]
fn can_win() {
    let mut connect4 = Connect4::make();
    connect4.board.place(4);
    connect4.board.place(5);
    connect4.board.place(4);
    connect4.board.place(5);
    connect4.board.place(4);
    connect4.board.place(5);
    let placement = ai::get_column_placement(connect4.board);
    assert_eq!(4, placement);
}


