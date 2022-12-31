
#[path = "../src/lib.rs"]
mod lib;
use lib::Connect4;

use lib::ai::evaluator::Evaluator;

#[test]
fn can_evaluate() {
    let mut connect4 = Connect4::make();
    connect4.board.place(0);
    connect4.board.place(1);
    connect4.board.place(0);
    connect4.board.place(1);
    connect4.board.place(0);
    connect4.board.place(1);
    let mut scorer = Evaluator::new(connect4.board);
    let score = scorer.get_score();
    assert_eq!(0, score);
}