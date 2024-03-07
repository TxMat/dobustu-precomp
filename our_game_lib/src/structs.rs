use std::error::Error;
use std::fmt::Display;
use board::Board;

#[derive(PartialEq, Eq)]
pub enum Result {
    Win,
    Lose,
    Intermediate(Vec<Board>)
}

#[derive(Debug)]
pub enum GameError {
    IllegalMove,
    OutOfBounds,
    NoPiece,
    PieceNotInBoard,
    NotYourPiece,
    GameOver,
    Unknown
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameError::IllegalMove => write!(f, "Illegal move"),
            GameError::OutOfBounds => write!(f, "Out of bounds"),
            GameError::NoPiece => write!(f, "No piece"),
            GameError::PieceNotInBoard => write!(f, "Piece not in board"),
            GameError::NotYourPiece => write!(f, "Not your piece"),
            GameError::GameOver => write!(f, "Game over"),
            GameError::Unknown => write!(f, "Unknown error")
        }
    }
}

impl Error for GameError {}