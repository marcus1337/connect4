extern crate lazy_static;

use super::board::Board;
use super::board::GameType;

pub mod library_singleton {
    use lazy_static::lazy_static;
    use std::fmt;
    use std::sync::{Arc, Mutex};
    use super::*;

    lazy_static! {
        static ref INSTANCE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    }

    pub struct State {
        pub board: Board,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.board)
        }
    }

    impl State {
        fn new() -> State {
            State {
                board: Board::new(GameType::ConnectFour),
            }
        }

        pub fn instance() -> Arc<Mutex<State>> {
            INSTANCE.clone()
        }
    }
}
