// Importing the necessary modules and structs for the Piece struct
use moves::Move;
use moves::{MOVE_CHICK, MOVE_ELEPHANT, MOVE_GIRAFFE, MOVE_HEN, MOVE_LION};

/// Represents a piece in the game.
/// Each piece has a type and a color.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    // The type of the piece (Chick, Giraffe, Elephant, Lion, Hen)
    pub piece_type: PieceType,
    // The color of the piece (White, Black)
    pub color: Color,
}

/// Represents the color of a piece.
/// Each piece can be either White or Black.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    White,
    Black,
}

/// Represents the type of a piece.
/// Each piece can be of type Chick, Giraffe, Elephant, Lion, or Hen.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceType {
    Chick,
    Giraffe,
    Elephant,
    Lion,
    Hen,
}

impl Piece {
    /// Creates a new piece with the given type and color.
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    pub fn show(&self) -> char {
        match self.color {
            Color::White => self.piece_type.show(),
            Color::Black => self.piece_type.show().to_ascii_uppercase(),
        }
    }

    pub fn is_move_valid(&self, move_: &Move) -> bool {
        self.piece_type.is_move_valid(move_)
    }
}

impl PieceType {
    /// Returns the moves that the piece can make.
    /// The moves are determined by the type of the piece.
    pub fn moves(&self) -> &[&Move] {
        match self {
            PieceType::Chick => MOVE_CHICK,
            PieceType::Giraffe => MOVE_GIRAFFE,
            PieceType::Elephant => MOVE_ELEPHANT,
            PieceType::Lion => MOVE_LION,
            PieceType::Hen => MOVE_HEN,
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
        match self {
            PieceType::Chick => 'c',
            PieceType::Giraffe => 'g',
            PieceType::Elephant => 'e',
            PieceType::Lion => 'l',
            PieceType::Hen => 'h',
        }
    }
}
