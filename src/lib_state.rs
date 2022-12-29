extern crate lazy_static;

use super::board::Board;

pub mod library_singleton {
    use lazy_static::lazy_static;
    use std::fmt;
    use std::sync::{Arc, Mutex};
    use super::Board;

    lazy_static! {
        static ref INSTANCE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    }

    pub struct State {
        pub data: i32,
        pub board: Board,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.data)
        }
    }

    impl State {
        fn new() -> State {
            State {
                data: 42,
                board: Board::new(),
            }
        }

        pub fn instance() -> Arc<Mutex<State>> {
            INSTANCE.clone()
        }
    }
}
