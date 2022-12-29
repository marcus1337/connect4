extern crate libc;

mod board;
mod lib_state;
use lib_state::library_singleton::State;

#[no_mangle]
pub fn add(value: i32) -> i32 {
    //let state = State::instance();
    //let mut state_data = state.lock().unwrap();
    0
}

#[no_mangle]
pub fn print_line_lib_state() {
    let state = State::instance();
    let state_data = state.lock().unwrap();
    println!("Singleton board data:");
    println!("{}", state_data.board);
}




