extern crate lazy_static;

pub mod library_singleton{
    use std::sync::{Arc, Mutex};
    use lazy_static::lazy_static;
    use std::fmt;

    lazy_static! {
        static ref INSTANCE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    }
    
    pub struct State {
        pub data: i32,
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.data)
        }
    }
    
    impl State {
        fn new() -> State {
            State { data: 42 }
        }
    
        pub fn instance() -> Arc<Mutex<State>> {
            INSTANCE.clone()
        }
    }
}

