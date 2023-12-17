use std::io;
use crate::chessboard::{Color, Chessboard};
use crate::cpu_player::Move as OtherMove;

pub struct HumanPlayer;

impl HumanPlayer {
    pub fn new() -> HumanPlayer{
        HumanPlayer
    }
    pub fn make_move(&mut self, chessboard: &Chessboard) -> Move{
        loop {
            println!("Enter your move (such as: a2 to a4):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let from = (
                7 - input.chars().nth(1).unwrap().to_digit(10).unwrap() as usize,
                (input.chars().nth(0).unwrap() as u8 - b'a') as usize,
                );
            let to = (
                7 - input.chars().nth(4).unwrap().to_digit(10).unwrap() as usize,
                (input.chars().nth(3).unwrap() as u8 - b'a') as usize,
                );

            if chessboard.is_move_valid(from,to){
                return Move {from, to};
            } else {
                println!("Invalid move!");
            }
        }
    }

}

pub struct Move{
    pub from: (usize, usize),
    pub to: (usize, usize),
}