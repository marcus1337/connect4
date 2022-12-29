extern crate libc;

pub mod board;
use board::Board;

#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}

#[no_mangle]
pub fn hello_from_rust() {
    println!("woot22");
}




