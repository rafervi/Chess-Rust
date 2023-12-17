use crate::chessboard::{Color,Chessboard};

pub struct CpuPlayer;

impl CpuPlayer {
    pub fn new() -> CpuPlayer{
        CpuPlayer
    }

    pub fn make_move(&mut self, chessboard: &Chessboard) -> Move{
        let from = (0, 0);
        let to = (0, 1);

        Move {from, to}
    }
}

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}