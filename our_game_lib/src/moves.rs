#[derive(PartialEq, Eq)]
pub struct Move {
    x: i8,
    y: i8
}

// move aliases
const MOVE_NW : &'static Move = &Move {x: -1, y:  1};
const MOVE_N  : &'static Move = &Move {x:  0, y:  1};
const MOVE_NE : &'static Move = &Move {x:  1, y:  1};
const MOVE_W  : &'static Move = &Move {x: -1, y:  0};
const MOVE_E  : &'static Move = &Move {x:  1, y:  0};
const MOVE_SW : &'static Move = &Move {x: -1, y: -1};
const MOVE_S  : &'static Move = &Move {x:  0, y: -1};
const MOVE_SE : &'static Move = &Move {x:  1, y: -1};

// possibles moves by types
pub const MOVE_DUMMY    : &'static [&'static Move] = &[];
pub const MOVE_LION     : &'static [&'static Move] = &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E, MOVE_SW, MOVE_S, MOVE_SE];
pub const MOVE_ELEPHANT : &'static [&'static Move] = &[MOVE_NW,         MOVE_NE,                 MOVE_SW,         MOVE_SE];
pub const MOVE_GIRAFFE  : &'static [&'static Move] = &[         MOVE_N,          MOVE_W, MOVE_E,          MOVE_S         ];
pub const MOVE_CHICK    : &'static [&'static Move] = &[         MOVE_N                                                   ];
pub const MOVE_HEN      : &'static [&'static Move] = &[MOVE_NW, MOVE_N, MOVE_NE, MOVE_W, MOVE_E,          MOVE_S         ];