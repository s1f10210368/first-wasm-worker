use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Othello {
    board: [[i8; 8]; 8],
    current_player: i8,
}

#[wasm_bindgen]
impl Othello {

    pub fn new() -> Othello {
        let mut board = [[0; 8]; 8];
        board[3][3] = 1;
        board[4][4] = 1;
        board[3][4] = -1; 
        board[4][3] = -1;

        Othello {
            board,
            current_player: 1,  
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize) {
        // 石を置く処理
    }

    pub fn get_current_player(&self) -> i8 {
        self.current_player
    }

}


_