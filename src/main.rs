
mod chessboard;
mod cpu_player;
mod human_player;

use crate::chessboard::Color;
use crate::chessboard::Chessboard;
use crate::cpu_player::CpuPlayer;
use crate::human_player::HumanPlayer;

fn main() {
    let mut chessboard = Chessboard::new();
    let mut cpu_player = CpuPlayer::new();
    let mut human_player = HumanPlayer::new();

    loop {
        println!("Current Chessboard:");
        chessboard.print();

        let human_move = human_player.make_move(&chessboard);
        chessboard.make_move(human_move.from, human_move.to);

        if chessboard.is_checkmate(Color::Black){
            println!("Checkmate! You win!");
            break;
        } else if chessboard.is_stalemate(Color::Black) {
            println!("Stalemate! It's a draw!");
            break;
        }
        let cpu_move = cpu_player.make_move(&chessboard);
        chessboard.make_move(cpu_move.from, cpu_move.to);

        if chessboard.is_checkmate (Color::White){
            println!("CPU wins!");
        } else if chessboard.is_stalemate(Color::White) {
            println!("Stalemate! It's a draw!");
            break;
        }
    }
}
