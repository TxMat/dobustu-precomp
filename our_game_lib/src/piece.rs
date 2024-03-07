// Importing the necessary modules and structs for the Piece struct
use moves::{MOVE_CHICK, MOVE_ELEPHANT, MOVE_GIRAFFE, MOVE_HEN, MOVE_LION, MOVE_DUMMY};
use moves::Move;

/// Represents a piece in the game.
/// Each piece has a type and a color.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    // The type of the piece (Chick, Giraffe, Elephant, Lion, Hen)
    pub(crate) piece_type: Type,
    // The color of the piece (White, Black)
    pub(crate) color: Color
}

/// Represents the color of a piece.
/// Each piece can be either White or Black.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    White,
    Black
}

/// Represents the type of a piece.
/// Each piece can be of type Chick, Giraffe, Elephant, Lion, or Hen.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Chick,
    Giraffe,
    Elephant,
    Lion,
    Hen
}

impl Piece {

    /// Creates a new piece with the given type and color.
    pub fn new(piece_type: Type, color: Color) -> Piece {
        Piece {
            piece_type,
            color
        }
    }

    /// Returns the moves that the piece can make.
    /// The moves are determined by the type of the piece.
    pub fn moves(&self) -> &'static [&'static Move] {
        match self.piece_type {
            Type::Chick => MOVE_CHICK,
            Type::Giraffe => MOVE_GIRAFFE,
            Type::Elephant => MOVE_ELEPHANT,
            Type::Lion => MOVE_LION,
            Type::Hen => MOVE_HEN
        }
    }
    
    /// Checks if a given move is valid for the piece.
    /// A move is valid if it is in the list of moves that the piece can make.
    pub fn is_move_valid(&self, move_: &Move) -> bool {
        self.moves().contains(&move_)
    }

    /// Returns a character representation of the piece.
    /// The character is determined by the type and color of the piece.
    pub fn show(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                Type::Chick => 'c',
                Type::Giraffe => 'g',
                Type::Elephant => 'e',
                Type::Lion => 'l',
                Type::Hen => 'h'
            },
            Color::Black => match self.piece_type {
                Type::Chick => 'C',
                Type::Giraffe => 'G',
                Type::Elephant => 'E',
                Type::Lion => 'L',
                Type::Hen => 'H'
            }
        }
    }
}