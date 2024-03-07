/// Represents a move in the game.
/// Each move has an x and y coordinate.
#[derive(PartialEq, Eq)]
pub struct Move {
    // The x coordinate of the move
    pub(crate) x: i8,
    // The y coordinate of the move
    pub(crate) y: i8
}

// Aliases for different types of moves
// MOVE_NW represents a move to the north-west
pub const MOVE_NW : &'static Move = &Move {x: -1, y:  1};
// MOVE_N represents a move to the north
pub const MOVE_N  : &'static Move = &Move {x:  0, y:  1};
// MOVE_NE represents a move to the north-east
pub const MOVE_NE : &'static Move = &Move {x:  1, y:  1};
// MOVE_W represents a move to the west
pub const MOVE_W  : &'static Move = &Move {x: -1, y:  0};
// MOVE_E represents a move to the east
pub const MOVE_E  : &'static Move = &Move {x:  1, y:  0};
// MOVE_SW represents a move to the south-west
pub const MOVE_SW : &'static Move = &Move {x: -1, y: -1};
// MOVE_S represents a move to the south
pub const MOVE_S  : &'static Move = &Move {x:  0, y: -1};
// MOVE_SE represents a move to the south-east
pub const MOVE_SE : &'static Move = &Move {x:  1, y: -1};

// Possible moves for each type of piece
// MOVE_DUMMY is an empty array, representing no possible moves
pub const MOVE_DUMMY    : &'static [&'static Move] = &[];
// MOVE_LION represents all possible moves for a Lion piece
pub const MOVE_LION     : &'static [&'static Move] = &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E, MOVE_SW, MOVE_S, MOVE_SE];
// MOVE_ELEPHANT represents all possible moves for an Elephant piece
pub const MOVE_ELEPHANT : &'static [&'static Move] = &[MOVE_NW,         MOVE_NE,                 MOVE_SW,         MOVE_SE];
// MOVE_GIRAFFE represents all possible moves for a Giraffe piece
pub const MOVE_GIRAFFE  : &'static [&'static Move] = &[         MOVE_N,          MOVE_W, MOVE_E,          MOVE_S         ];
// MOVE_CHICK represents all possible moves for a Chick piece
pub const MOVE_CHICK    : &'static [&'static Move] = &[         MOVE_N                                                   ];
// MOVE_HEN represents all possible moves for a Hen piece
pub const MOVE_HEN      : &'static [&'static Move] = &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E,          MOVE_S         ];