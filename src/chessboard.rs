use std::ffi::FromBytesUntilNulError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Piece {
    Empty,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub piece: Piece,
    pub color: Color,
}
pub struct Chessboard {
    pub board: [[Square; 8]; 8],
}

impl Chessboard {
    pub fn new() -> Chessboard {
        let mut board = [[Square {piece: Piece::Empty, color: Color::White}; 8]; 8];

        for i in 0..8 {
            board[1][i] = Square {
                piece: Piece::Pawn,
                color: Color::White,
            };
            board[6][i] = Square {
                piece:Piece::Pawn,
                color:Color::Black,
            };
        }

        let pieces_order = [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::King];

        for  &color in &[Color::White, Color::Black] {
            for (i, &piece) in pieces_order.iter().enumerate(){
                board[if color == Color::White{
                    0
                }else {
                    7
                }]
                [i] = Square {piece,color};
            }
        }
        Chessboard { board }
    }
    pub fn print(&self) {

        for row in self.board.iter(){
            for square in row.iter() {
                print!("{}", match square.piece {
                    Piece::Empty=> ".",
                    Piece::Pawn=> "P",
                    Piece::Bishop => "B",
                    Piece::King => "K",
                    Piece::Knight => "N",
                    Piece::Rook => "R",
                    Piece::Queen => "Q",
                });

            }
            println!();
        }
    }
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)){
        let piece = self.board[from.0][from.1];
        self.board[to.0][to.1] = piece;
        self.board[from.0][from.1] = Square {
            piece: Piece::Empty,
            color: Color::White,
        };
    }

    pub fn is_move_valid (&self, from: (usize, usize), to: (usize, usize)) -> bool {
        if from.0 >= 8 || from.1 >= 8 || to.0 >= 8 || to.1 >= 8{
            return false;
        }

        if self.board[from.0][from.1].color != Color::White {
            return false;
        }
        true
    }
    pub fn is_checkmate(&self, color: Color) -> bool{
        false
    }
    pub fn is_stalemate(&self, color: Color) -> bool{
        false
    }
}
