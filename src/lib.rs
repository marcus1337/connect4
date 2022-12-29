extern crate libc;

mod lib_state;
use lib_state::library_singleton::State;

#[no_mangle]
pub fn add(value: i32) -> i32 {
    let state = State::instance();
    let mut stateData = state.lock().unwrap();
    stateData.data += value;
    stateData.data
}

#[no_mangle]
pub fn sub(value: i32) -> i32 {
    let state = State::instance();
    let mut stateData = state.lock().unwrap();
    stateData.data -= value;
    stateData.data
}

#[no_mangle]
pub fn printLineLibState() {
    let state = State::instance();
    let stateData = state.lock().unwrap();
    println!("Singleton data: {}", stateData);
}




