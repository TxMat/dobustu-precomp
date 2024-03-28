#![cfg_attr(rustfmt, rustfmt_skip)]

use std::fmt::Display;

/// Represents a move in the game.
/// Each move has an x and y coordinate.
#[derive(PartialEq, Eq, Debug)]
pub struct Move {
    // The x coordinate of the move
    pub(crate) x: i8,
    // The y coordinate of the move
    pub(crate) y: i8,
}

// Aliases for different types of moves
pub const MOVE_NW: &Move = &Move { x: -1, y: 1 };
pub const MOVE_N: &Move = &Move { x: 0, y: 1 };
pub const MOVE_NE: &Move = &Move { x: 1, y: 1 };
pub const MOVE_W: &Move = &Move { x: -1, y: 0 };
pub const MOVE_E: &Move = &Move { x: 1, y: 0 };
pub const MOVE_SW: &Move = &Move { x: -1, y: -1 };
pub const MOVE_S: &Move = &Move { x: 0, y: -1 };
pub const MOVE_SE: &Move = &Move { x: 1, y: -1 };

// Possible moves for each type of piece
pub const MOVE_DUMMY: &[&Move] = &[];
pub const MOVE_LION:     &[&Move] = &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E, MOVE_SW, MOVE_S, MOVE_SE];
pub const MOVE_ELEPHANT: &[&Move] = &[MOVE_NW,         MOVE_NE,                 MOVE_SW,         MOVE_SE];
pub const MOVE_GIRAFFE: &[&Move] =  &[         MOVE_N,          MOVE_W, MOVE_E,          MOVE_S         ];
pub const MOVE_CHICK:   &[&Move] =  &[         MOVE_N                                                   ];
pub const MOVE_HEN:     &[&Move] =  &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E,          MOVE_S         ];

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.x, self.y) {
            (-1, 1) => write!(f, "NW"),
            (0, 1) => write!(f, "N"),
            (1, 1) => write!(f, "NE"),
            (-1, 0) => write!(f, "W"),
            (0, 0) => write!(f, "X"),
            (1, 0) => write!(f, "E"),
            (-1, -1) => write!(f, "SW"),
            (0, -1) => write!(f, "S"),
            (1, -1) => write!(f, "SE"),
            _ => write!(f, "Unknown"),
        }
    }
}

// impl Move {
//     pub(crate) fn invert(&self) -> &Move {
//         match self {
//             MOVE_NW => MOVE_SE,
//             MOVE_N => MOVE_S,
//             MOVE_NE => MOVE_SW,
//             MOVE_W => MOVE_E,
//             MOVE_E => MOVE_W,
//             MOVE_SW => MOVE_NE,
//             MOVE_S => MOVE_N,
//             MOVE_SE => MOVE_NW,
//             _ => self,
//         }
//     }
// }