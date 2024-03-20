use board::Board;
use std::error::Error;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Intermediate(Vec<Board>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    X0Y0 = 0,
    X1Y0 = 1,
    X2Y0 = 2,
    X0Y1 = 3,
    X1Y1 = 4,
    X2Y1 = 5,
    X0Y2 = 6,
    X1Y2 = 7,
    X2Y2 = 8,
    X0Y3 = 9,
    X1Y3 = 10,
    X2Y3 = 11,
    Dead = 12,
}

impl PartialEq<(u8, u8)> for &Position {
    fn eq(&self, other: &(u8, u8)) -> bool {
        let (x, y) = <&Position>::into(*self);
        (x, y) == *other
    }
}

impl From<u8> for Position {
    fn from(n: u8) -> Self {
        match n {
            0 => Position::X0Y0,
            1 => Position::X1Y0,
            2 => Position::X2Y0,
            3 => Position::X0Y1,
            4 => Position::X1Y1,
            5 => Position::X2Y1,
            6 => Position::X0Y2,
            7 => Position::X1Y2,
            8 => Position::X2Y2,
            9 => Position::X0Y3,
            10 => Position::X1Y3,
            11 => Position::X2Y3,
            12 => Position::Dead,
            _ => panic!("Invalid position"),
        }
    }
}

impl Into<(u8, u8)> for &Position {
    fn into(self) -> (u8, u8) {
        match self {
            Position::X0Y0 => (0, 0),
            Position::X1Y0 => (1, 0),
            Position::X2Y0 => (2, 0),
            Position::X0Y1 => (0, 1),
            Position::X1Y1 => (1, 1),
            Position::X2Y1 => (2, 1),
            Position::X0Y2 => (0, 2),
            Position::X1Y2 => (1, 2),
            Position::X2Y2 => (2, 2),
            Position::X0Y3 => (0, 3),
            Position::X1Y3 => (1, 3),
            Position::X2Y3 => (2, 3),
            Position::Dead => (9, 9),
        }
    }
}

impl Into<(i8, i8)> for &Position {
    fn into(self) -> (i8, i8) {
        match self {
            Position::X0Y0 => (0, 0),
            Position::X1Y0 => (1, 0),
            Position::X2Y0 => (2, 0),
            Position::X0Y1 => (0, 1),
            Position::X1Y1 => (1, 1),
            Position::X2Y1 => (2, 1),
            Position::X0Y2 => (0, 2),
            Position::X1Y2 => (1, 2),
            Position::X2Y2 => (2, 2),
            Position::X0Y3 => (0, 3),
            Position::X1Y3 => (1, 3),
            Position::X2Y3 => (2, 3),
            Position::Dead => (9, 9),
        }
    }
}

impl Into<u8> for Position {
    fn into(self) -> u8 {
        match self {
            Position::X0Y0 => 0,
            Position::X1Y0 => 1,
            Position::X2Y0 => 2,
            Position::X0Y1 => 3,
            Position::X1Y1 => 4,
            Position::X2Y1 => 5,
            Position::X0Y2 => 6,
            Position::X1Y2 => 7,
            Position::X2Y2 => 8,
            Position::X0Y3 => 9,
            Position::X1Y3 => 10,
            Position::X2Y3 => 11,
            Position::Dead => 12,
        }
    }
}

impl Into<u8> for &Position {
    fn into(self) -> u8 {
        match self {
            Position::X0Y0 => 0,
            Position::X1Y0 => 1,
            Position::X2Y0 => 2,
            Position::X0Y1 => 3,
            Position::X1Y1 => 4,
            Position::X2Y1 => 5,
            Position::X0Y2 => 6,
            Position::X1Y2 => 7,
            Position::X2Y2 => 8,
            Position::X0Y3 => 9,
            Position::X1Y3 => 10,
            Position::X2Y3 => 11,
            Position::Dead => 12,
        }
    }
}

impl From<(u8, u8)> for Position {
    fn from(value: (u8, u8)) -> Self {
        match value {
            (0, 0) => Position::X0Y0,
            (1, 0) => Position::X1Y0,
            (2, 0) => Position::X2Y0,
            (0, 1) => Position::X0Y1,
            (1, 1) => Position::X1Y1,
            (2, 1) => Position::X2Y1,
            (0, 2) => Position::X0Y2,
            (1, 2) => Position::X1Y2,
            (2, 2) => Position::X2Y2,
            (0, 3) => Position::X0Y3,
            (1, 3) => Position::X1Y3,
            (2, 3) => Position::X2Y3,
            _ => panic!("Invalid position"),
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    OutOfBounds,
    PieceNotInBoard,
    NotYourPiece,
    GameOver,
    EmptyCemetary,
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
            GameError::EmptyCemetary => write!(f, "Empty Cemetary"),
        }
    }
}

impl Error for GameError {}
