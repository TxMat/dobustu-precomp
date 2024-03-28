use std::fmt::Display;

// Importing the necessary modules and structs for the Piece struct
use moves::Move;
use moves::{MOVE_CHICK, MOVE_ELEPHANT, MOVE_GIRAFFE, MOVE_HEN, MOVE_LION};

/// Represents a piece in the game.
/// Each piece has a type and a color.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub struct Piece(pub u8);

pub const EMPTY: Piece = Piece(0);
pub const LION_1: Piece = Piece(1);
pub const LION_2: Piece = Piece(2);
pub const ELEPHANT_1: Piece = Piece(3);
pub const ELEPHANT_2: Piece = Piece(4);
pub const GIRAFFE_1: Piece = Piece(5);
pub const GIRAFFE_2: Piece = Piece(6);
pub const CHICK_1: Piece = Piece(7);
pub const CHICK_2: Piece = Piece(8);
pub const HEN_1: Piece = Piece(9);
pub const HEN_2: Piece = Piece(10);

impl From<u8> for Piece {
    fn from(n: u8) -> Self {
        match n {
            0 => EMPTY,
            1 => LION_1,
            2 => LION_2,
            3 => ELEPHANT_1,
            4 => ELEPHANT_2,
            5 => GIRAFFE_1,
            6 => GIRAFFE_2,
            7 => CHICK_1,
            8 => CHICK_2,
            9 => HEN_1,
            10 => HEN_2,
            _ => panic!("Invalid piece"),
        }
    }
}

impl Piece {
    pub fn show(&self) -> char {
        let s = match self.0 {
            1 => 'l',
            2 => 'L',
            3 => 'e',
            4 => 'E',
            5 => 'g',
            6 => 'G',
            7 => 'c',
            8 => 'C',
            9 => 'h',
            10 => 'H',
            _ => ' ',
        };
        s
    }

    pub(crate) fn moves(&self) -> &'static [&'static Move] {
        match *self {
            LION_1 | LION_2 => MOVE_LION,
            ELEPHANT_1 | ELEPHANT_2 => MOVE_ELEPHANT,
            GIRAFFE_1 | GIRAFFE_2 => MOVE_GIRAFFE,
            CHICK_1 | CHICK_2 => MOVE_CHICK,
            HEN_1 | HEN_2 => MOVE_HEN,
            _ => panic!("Invalid piece"),
        }
    }

    pub fn is_mine(&self, is_player_1: bool) -> bool {
        self.0 != 0 && self.0 % 2 == is_player_1 as u8
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}
// impl Piece {
//     /// Creates a new piece with the given type and color.
//     pub fn new(piece_type: PieceType, color: Color, is_duplicated: bool) -> Piece {
//         Piece {
//             piece_type,
//             color,
//             is_duplicated,
//         }
//     }
//
//     pub fn show(&self) -> String {
//         let s = match self.color {
//             Color::White => self.piece_type.show(),
//             Color::Black => self.piece_type.show().to_ascii_uppercase(),
//         };
//         if self.is_duplicated {
//             return format!("{}*", s);
//         }
//         s.to_string()
//     }
//
//     pub fn is_move_valid(&self, move_: &Move) -> bool {
//         self.piece_type.is_move_valid(move_)
//     }
// }
//
// impl PieceType {
//     /// Returns the moves that the piece can make.
//     /// The moves are determined by the type of the piece.
//     pub fn moves(&self) -> &[&Move] {
//         match self {
//             PieceType::Chick => MOVE_CHICK,
//             PieceType::Giraffe => MOVE_GIRAFFE,
//             PieceType::Elephant => MOVE_ELEPHANT,
//             PieceType::Lion => MOVE_LION,
//             PieceType::Hen => MOVE_HEN,
//         }
//     }
//
//     /// Checks if a given move is valid for the piece.
//     /// A move is valid if it is in the list of moves that the piece can make.
//     pub fn is_move_valid(&self, move_: &Move) -> bool {
//         self.moves().contains(&move_)
//     }
//
//     /// Returns a character representation of the piece.
//     /// The character is determined by the type and color of the piece.
//     pub fn show(&self) -> char {
//         match self {
//             PieceType::Chick => 'c',
//             PieceType::Giraffe => 'g',
//             PieceType::Elephant => 'e',
//             PieceType::Lion => 'l',
//             PieceType::Hen => 'h',
//         }
//     }
// }
