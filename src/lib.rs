extern crate libc;

mod board;
mod lib_state;

use lib_state::singleton::State;

#[no_mangle]
pub fn connect4_print() {
    let state = State::instance();
    let state_data = state.lock().unwrap();
    println!("Singleton board data:");
    println!("{}", state_data.board);
}

