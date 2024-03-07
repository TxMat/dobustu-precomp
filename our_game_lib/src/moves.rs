#![cfg_attr(rustfmt, rustfmt_skip)]

/// Represents a move in the game.
/// Each move has an x and y coordinate.
#[derive(PartialEq, Eq)]
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