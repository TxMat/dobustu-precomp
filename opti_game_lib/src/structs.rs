use board::Board;
use next_move::NextMove;
use piece::Piece;
use std::error::Error;
use std::fmt::Display;
use std::ops::Add;

#[derive(PartialEq, Eq, Debug)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Intermediate(Vec<(NextMove, Board)>),
}

pub enum Calc<G> {
    GameResult(G),
    Proba((NextMove, f32)),
}

impl<T> Calc<T> {
    pub fn unwrap_some(&self) -> &T {
        match self {
            Calc::GameResult(val) => val,
            Calc::Proba(_) => panic!("No way"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
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

impl Position {
    pub fn is_winning_row_for_player(&self, is_player_1: bool) -> bool {
        match *self {
            Position::X0Y0 | Position::X1Y0 | Position::X2Y0 => !is_player_1,
            Position::X0Y3 | Position::X1Y3 | Position::X2Y3 => is_player_1,
            _ => return false,
        }
    }

    pub fn as_u8(&self) -> u8 {
        <Position as Into<u8>>::into(*self)
    }
}

impl From<u16> for Position {
    fn from(n: u16) -> Self {
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

// impl Into<(Piece, Position)> for NextMove {
//     fn into(self) -> (Piece, Position) {
//         let piece = (self.0 & 0xf0) >> 4;
//         let pos = self.0 & 0x0f;
//         (Piece::from(piece), Position::from(pos))
//     }
// }

// impl From<(Piece, Position)> for NextMove {
//     fn from(value: (Piece, Position)) -> Self {
//         let piece_pos: u8 = (value.0 .0 << 4) + <&Position as Into<u8>>::into(&value.1);
//         NextMove(piece_pos)
//     }
// }

// impl Display for NextMove {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let (piece, pos) = <NextMove as Into<(Piece, Position)>>::into(*self);
//         write!(f, "{} at {:?}", piece, pos)
//     }
// }

// let arr = self.0.to_be_bytes();
//         let pos_u8 = position as u8;
//         for a in arr.iter() {
//             let piece = (a & 0xf0) >> 4;
//             let pos = a & 0x0f;
//             if pos == pos_u8 {
//                 return Piece::from(piece);
//             }
//         }
