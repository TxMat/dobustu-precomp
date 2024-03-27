use std::cmp::Ordering;
use std::convert::TryInto;
/// todo divide the code in smaller functions
use std::error::Error;
use std::fmt::{Binary, LowerHex, UpperHex};
use std::hash::Hash;
use std::mem::{size_of, size_of_val};

use log::{error, info};

// Importing the necessary modules and structs for the Board struct
use moves::Move;
use piece::{
    Piece, CHICK_1, CHICK_2, ELEPHANT_1, ELEPHANT_2, EMPTY, GIRAFFE_1, GIRAFFE_2, HEN_1, HEN_2,
    LION_1, LION_2,
};
use structs::GameResult::{BlackWin, Intermediate, WhiteWin};
use structs::Position::{
    Dead, X0Y0, X0Y1, X0Y2, X0Y3, X1Y0, X1Y1, X1Y2, X1Y3, X2Y0, X2Y1, X2Y2, X2Y3,
};
use structs::{GameError, GameResult, Position};

// The `derive` attribute automatically implements the specified traits for the struct.
// Clone: Allows the struct to be duplicated.
// Copy: Allows the struct to be copied.
// PartialEq and Eq: Allows the struct to be compared for equality.
// Hash: Allows the struct to be used as a key in a HashMap.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct Board(pub u64);

impl Board {
    // Creates a new empty board.
    pub fn new_empty() -> Board {
        Board(0)
    }

    // Returns the piece at the given coordinates.
    pub fn get_at_pos_slow(&self, position: Position) -> Piece {
        // split the u64 into 8 8-bit pieces
        let arr = self.0.to_be_bytes();
        let pos_u8 = position as u8;
        for a in arr.iter() {
            let piece = (a & 0xf0) >> 4;
            let pos = a & 0x0f;
            if pos == pos_u8 {
                return Piece::from(piece);
            }
        }
        Piece(0)
    }

    // pub fn get_pos_from_piece(&self, piece: Piece) -> Position {
    //     let shift = piece.get_bit_shift();
    //     let mask = 0x0f << shift;
    //     let a = ((self.0 & mask) >> shift);
    //     Position::from(a as u8)
    // }

    pub fn show_hex(&self) {
        println!("{:X}", self.0);
    }

    // Places a piece at the given coordinates.
    // pub fn put(&mut self, position: Position, piece: Piece) {
    //     let piece_pos = (piece.0 << 4) + (position as u8);
    //     info!("new piece pos: {:X}", piece_pos);
    //     let shift = piece.get_bit_shift();
    //
    //     let mask = 0xff << shift;
    //
    //     self.0 = (self.0 & !mask) | (((piece_pos as u64) << shift));
    //
    // }

    // Places a piece at the given coordinates.
    pub fn put_state(&mut self, state: [(Position, Piece); 8]) {
        let mut count: u8 = 0;
        for (pos, piece) in state.iter() {
            let piece_pos: u8 = (piece.0 << 4) + <&Position as Into<u8>>::into(pos);
            //info!("new piece pos: {:X}", piece_pos);
            let shift = count * 8;

            let mask = 0xff << shift;

            self.0 = (self.0 & !mask) | ((piece_pos as u64) << shift);
            count += 1;
        }
    }

    pub fn get_state(&self) -> [(Position, Piece); 8] {
        let arr = self.0.to_le_bytes();
        let mut i: usize = 0;
        let mut result = [(X0Y0, EMPTY); 8];
        for piece_pos in arr.iter() {
            let piece = (piece_pos & 0xf0) >> 4;
            let pos = piece_pos & 0x0f;
            let pos = Position::from(pos);
            let piece = Piece::from(piece);
            result[i] = (pos, piece);
            i += 1;
        }
        result
    }

    pub fn debug_show_board(&self) {
        let state = self.get_state();
        let mut count: u8 = 0;
        for (pos, piece) in state.iter() {
            //info!("piece: {:?}, pos: {:?}", piece, pos);
            //info!("count: {:?}", count);
            count += 1;
        }
    }

    pub fn get_state_processed(&self) -> [[Piece; 3]; 4] {
        let state = self.get_state();
        let mut state_processed: [[Piece; 3]; 4] = [
            [EMPTY, EMPTY, EMPTY], // y3
            [EMPTY, EMPTY, EMPTY], // y2
            [EMPTY, EMPTY, EMPTY], // y1
            [EMPTY, EMPTY, EMPTY], // y0
        ];
        for (pos, piece) in state.iter() {
            if *pos == Position::Dead {
                continue;
            }
            let x = *pos as u8 % 3;
            let y = *pos as u8 / 3;
            state_processed[y as usize][x as usize] = *piece;
        }
        state_processed
    }

    pub fn get_state_processed_from_state(state: [(Position, Piece); 8]) -> [[Piece; 3]; 4] {
        let mut state_processed: [[Piece; 3]; 4] = [
            [EMPTY, EMPTY, EMPTY], // y3
            [EMPTY, EMPTY, EMPTY], // y2
            [EMPTY, EMPTY, EMPTY], // y1
            [EMPTY, EMPTY, EMPTY], // y0
        ];
        for (pos, piece) in state.iter() {
            if *pos == Position::Dead {
                continue;
            }
            let x = *pos as u8 % 3;
            let y = *pos as u8 / 3;
            state_processed[y as usize][x as usize] = *piece;
        }
        state_processed
    }

    //1 true 2 false
    pub fn get_next_states(&self, is_player_1: bool) -> GameResult {
        let state = self.get_state();
        let state_processed = Self::get_state_processed_from_state(state);
        let mut boards = vec![];

        for (pos, piece) in state.iter() {
            if !piece.is_mine(is_player_1) {
                continue;
            }
            if *pos == Position::Dead {
                for y in 0..4 {
                    for x in 0..3 {
                        // if the position is not empty skip
                        if state_processed[y][x].0 == 0 {
                            let new_pos = Position::from((x as u8, y as u8));
                            boards.push(Self::compute_parachuted_board(&state, &new_pos, piece));
                        }
                    }
                }
                continue;
            }
            let moves = piece.moves();
            for m in moves {
                let converted_pos = <&Position as Into<(i8, i8)>>::into(pos);
                let new_pos = match is_player_1 {
                    true => (converted_pos.0 + m.x, converted_pos.1 + m.y),
                    false => (converted_pos.0 - m.x, converted_pos.1 - m.y),
                };
                if new_pos.0 < 0 || new_pos.0 > 2 || new_pos.1 < 0 || new_pos.1 > 3 {
                    continue;
                }
                let new_pos: (u8, u8) = (new_pos.0 as u8, new_pos.1 as u8);
                let piece_on_new_pos = state_processed[new_pos.1 as usize][new_pos.0 as usize];
                if !piece_on_new_pos.is_mine(is_player_1) {
                    if piece_on_new_pos == LION_1 && !is_player_1 {
                        return GameResult::BlackWin;
                    }
                    if piece_on_new_pos == LION_2 && is_player_1 {
                        return GameResult::WhiteWin;
                    }
                    boards.push(Self::compute_new_board(
                        &state,
                        pos,
                        &Position::from(new_pos),
                    ));
                }
            }
        }
        Intermediate(boards)
    }

    // todo continuer ici
    pub fn compute_new_board(
        state: &[(Position, Piece); 8],
        current_pos: &Position,
        new_pos: &Position,
    ) -> Board {
        let mut new_state = state.clone();
        for (pos, piece) in new_state.iter_mut() {
            if pos == new_pos {
                if *piece != EMPTY {
                    *pos = Position::Dead;
                    *piece = match *piece {
                        HEN_1 | CHICK_1 => CHICK_2,
                        HEN_2 | CHICK_2 => CHICK_1,
                        ELEPHANT_1 => ELEPHANT_2,
                        ELEPHANT_2 => ELEPHANT_1,
                        GIRAFFE_1 => GIRAFFE_2,
                        GIRAFFE_2 => GIRAFFE_1,
                        _ => unreachable!("Should not happen"),
                    };
                }
            } else if pos == current_pos {
                *pos = *new_pos;

                if (*piece == CHICK_1 && (*new_pos as u8) > 8) {
                    *piece = HEN_1;
                } else if (*piece == CHICK_2 && (*new_pos as u8) < 3) {
                    *piece = HEN_2;
                }
            }
        }
        let mut b = Board::new_empty();
        b.put_state(new_state);
        b
    }

    pub fn compute_parachuted_board(
        state: &[(Position, Piece); 8],
        new_pos: &Position,
        piece: &Piece,
    ) -> Board {
        let mut new_state = state.clone();
        for (pos, p) in new_state.iter_mut() {
            if *p == *piece && *pos == Dead {
                *pos = *new_pos;
                break;
            }
        }
        if new_state == *state {
            panic!("Unchancged state");
        }
        let mut b = Board::new_empty();
        b.put_state(new_state);
        b
    }

    // ugly
    pub fn debug_show_board_2(&self) {
        println!("##############");

        let state = self.get_state();
        let mut white_cemetery = vec![];
        let mut black_cemetery = vec![];

        let mut state_processed: [[Piece; 3]; 4] = [
            [EMPTY, EMPTY, EMPTY], // y3
            [EMPTY, EMPTY, EMPTY], // y2
            [EMPTY, EMPTY, EMPTY], // y1
            [EMPTY, EMPTY, EMPTY], // y0
        ];

        for (pos, piece) in state.iter() {
            if pos == &Position::Dead {
                match *piece {
                    CHICK_1 | GIRAFFE_1 | ELEPHANT_1 | LION_1 | HEN_1 => {
                        white_cemetery.push(*piece);
                    }
                    CHICK_2 | GIRAFFE_2 | ELEPHANT_2 | LION_2 | HEN_2 => {
                        black_cemetery.push(*piece);
                    }
                    _ => unreachable!("Should not happen"),
                }
                continue;
            }
            let x = *pos as u8 % 3;
            let y = *pos as u8 / 3;
            state_processed[y as usize][x as usize] = *piece;
        }

        fn print_row_nb() {
            print!("    0 1 2")
        }

        for y in (0..6).rev() {
            match y {
                0 => print_row_nb(),
                5 => print_row_nb(),
                _ => {
                    let y_new = y - 1;
                    for x in 0..5 {
                        match x {
                            0 => print!("{} | ", y_new),
                            4 => print!("| {}", y_new),
                            _ => print!(
                                "{} ",
                                state_processed[y_new as usize][x - 1 as usize].show()
                            ),
                        }
                    }
                }
            }

            println!();
        }
        println!("##############");
        println!(
            "White cemetery: {:?}",
            white_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
        println!(
            "Black cemetery: {:?}",
            black_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
        println!();
    }

    // Removes the piece at the given coordinates.
    // pub fn del(&mut self, x: usize, y: usize) {
    //     self.board[y][x] = None;
    // }

    // Initializes the board with the default setup.
    pub fn init() -> Board {
        let mut b = Board::new_empty();
        let state = [
            (X1Y0, LION_1),
            (X1Y3, LION_2),
            (X0Y0, ELEPHANT_1),
            (X2Y3, ELEPHANT_2),
            (X2Y0, GIRAFFE_1),
            (X0Y3, GIRAFFE_2),
            (X1Y1, CHICK_1),
            (X1Y2, CHICK_2),
        ];
        b.put_state(state);
        b
    }
}
