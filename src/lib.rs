
use self::board::Board;
extern crate libc;
mod board;

//cbindgen --output target/release/connect4.h

#[repr(C)]
struct Connect4 {
    board: Board,
}

impl Connect4 {

    #[no_mangle]
    pub extern "C" fn make() -> Self {
        Self {
            board: Board::new()
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
    
}
