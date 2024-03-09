use board::Board;
use std::error::Error;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum Result {
    Win,
    Lose,
    Intermediate(Vec<Board>),
}

#[derive(Debug)]
pub enum GameError {
    OutOfBounds,
    PieceNotInBoard,
    NotYourPiece,
    GameOver,
    CantMoveAnywhere,
    InavlidPiece,
    InvalidMove,
    IllegalMove,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameError::IllegalMove => write!(f, "Illegal move"),
            GameError::OutOfBounds => write!(f, "Out of bounds"),
            GameError::PieceNotInBoard => write!(f, "Piece not in board"),
            GameError::NotYourPiece => write!(f, "Not your piece"),
            GameError::GameOver => write!(f, "Game over"),
            GameError::CantMoveAnywhere => write!(f, "Can't move anywhere"),
            GameError::InavlidPiece => write!(f, "Invalid piece"),
            GameError::InvalidMove => write!(f, "Invalid move"),
        }
    }
}

impl Error for GameError {}
