extern crate libc;

mod board;
mod lib_state;
use lib_state::singleton::State;

#[no_mangle]
pub fn connect4_print() {
    let state = State::instance();
    let state_data = state.lock().unwrap();
    println!("{}", state_data.board);
}

#[no_mangle]
pub fn connect4_place(col: i32){
    let state = State::instance();
    let mut state_data = state.lock().unwrap();
    state_data.board.place(col as usize);
}

