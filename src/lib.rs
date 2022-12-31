
use self::board::Board;
extern crate libc;
pub mod board;
pub mod ai;

//cbindgen --output target/release/connect4.h

#[repr(C)]
pub struct Connect4 {
    pub board: Board,
}

impl Connect4 {

    #[no_mangle]
    pub extern "C" fn make() -> Self {
        Self {
            board: Board::new(),
        }
    }

    #[no_mangle]
    pub extern "C" fn reset(&mut self) {
        self.board.reset();
    }

    #[no_mangle]
    pub extern "C" fn print(&mut self) {
        println!("{}", self.board);
    }

    //#[no_mangle]
    //pub extern "C" fn get_ai_move(&mut self) -> i32 {
    //    0
    //}
    //pub extern "C" fn apply_ai_move(&mut self) {
    //}
    
}
