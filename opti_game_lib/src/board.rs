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
// PartialEq and Eq: Allows the struct to be compared for equality.
// Hash: Allows the struct to be used as a key in a HashMap.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board(u64);

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

//     // Finds a piece on the board and returns its coordinates.
//     pub fn find_piece_on_board(&self, piece: &Piece) -> Result<(usize, usize), Box<dyn Error>> {
//         for y in 0..4 {
//             for x in 0..3 {
//                 if self.get(x, y) == Some(*piece) {
//                     return Ok((x, y));
//                 }
//             }
//         }
//         Err(GameError::PieceNotInBoard.into())
//     }
//
//     pub fn get_curent_player_piece_list(&self) -> Vec<Piece> {
//         let mut result = Vec::new();
//         for y in 0..4 {
//             for x in 0..3 {
//                 if let Some(p) = self.get(x, y) {
//                     if p.color == self.get_turn() {
//                         result.push(p);
//                     }
//                 }
//             }
//         }
//         result
//     }
//
//     pub fn get_legal_move_list_for_piece<'a>(
//         &'a self,
//         piece: &'a Piece,
//     ) -> Result<Vec<&Move>, Box<dyn Error>> {
//         let mut result = Vec::new();
//         let piece_pos = self.find_piece_on_board(piece)?;
//
//         let moves: Vec<&Move> = piece
//             .piece_type
//             .moves()
//             .iter()
//             .map(|m| if piece.color == Black { m.invert() } else { *m })
//             .collect();
//
//         for m in moves {
//             let (x, y) = (piece_pos.0 as i8 + m.x, piece_pos.1 as i8 + m.y);
//             if x > 2 || y > 3 || x < 0 || y < 0 {
//                 continue;
//             }
//             let target_pos = (x as usize, y as usize);
//             let piece_on_the_way = self.get(target_pos.0, target_pos.1);
//             if let Some(p) = piece_on_the_way {
//                 if p.color != piece.color {
//                     result.push(m);
//                 }
//             } else {
//                 result.push(m);
//             }
//         }
//         Ok(result)
//     }
//
//     pub fn get_curent_player_cemetery(&self) -> Vec<Piece> {
//         match self.get_turn() {
//             White => self.white_cemetery.clone(),
//             Black => self.black_cemetery.clone(),
//         }
//     }
//
//     // Moves a piece on the board if the move is valid.
//     // Returns a `Result` type which is an enum that can either be `Ok` or `Err`.
//     // `Ok` indicates a successful operation and `Err` indicates an error.
//     pub fn move_piece(&mut self, piece: &mut Piece, move_: &Move) -> Result<(), Box<dyn Error>> {
//         // Check if the move is valid for the piece.
//         if self.get_legal_move_list_for_piece(piece)?.contains(&&move_) {
//             // Set the color of the piece to the current turn's color.
//             piece.color = self.get_turn();
//             // Find the piece on the board.
//             let piece_pos = self.find_piece_on_board(piece)?;
//
//             // Calculate the new coordinates for the piece.
//             let (x, y) = (piece_pos.0 as i8 + move_.x, piece_pos.1 as i8 + move_.y);
//             // Check if the new coordinates are out of bounds.
//             if x > 2 || y > 3 || x < 0 || y < 0 {
//                 return Err(GameError::OutOfBounds.into());
//             }
//
//             // Get the color of the current player.
//             let current_player_color = self.get_turn();
//
//             // Get the piece at the target position.
//             let target_pos = (x as usize, y as usize);
//             let piece_on_the_way = self.get(target_pos.0, target_pos.1);
//
//             // If there is a piece at the target position, check if it's the same color as the current player.
//             if let Some(mut p) = piece_on_the_way {
//                 if p.color == current_player_color {
//                     return Err(GameError::IllegalMove.into());
//                 }
//                 // If the piece at the target position is a Lion, the current player wins.
//                 match p.piece_type {
//                     Lion => self.winner = Some(current_player_color),
//                     PieceType::Hen => p.piece_type = Chick,
//                     _ => (),
//                 }
//                 if p.is_duplicated {
//                     p.is_duplicated = false;
//                 }
//                 // Add the piece at the target position to the current player's cemetery.
//                 let cemetery = match current_player_color {
//                     White => &mut self.white_cemetery,
//                     Black => &mut self.black_cemetery,
//                 };
//                 p.color = current_player_color;
//                 cemetery.push(p);
//             }
//
//             // Remove the piece from its current position.
//             self.del(piece_pos.0, piece_pos.1);
//
//             // If the piece is a Chick and it reaches the last row, it becomes a Hen.
//             if piece.piece_type == Chick && (target_pos.1 == 0 || target_pos.1 == 3) {
//                 let mut hen = Piece::new(PieceType::Hen, current_player_color, false);
//                 if self.find_piece_on_board(&hen).is_ok() {
//                     hen.is_duplicated = true;
//                 }
//                 self.put(target_pos.0, target_pos.1, &hen);
//             } else {
//                 // Otherwise, move the piece to the target position.
//                 self.put(target_pos.0, target_pos.1, piece);
//             }
//
//             // If the piece is a Lion and it reaches the last row, the game is over.
//             if piece.piece_type == Lion
//                 && ((current_player_color == White && target_pos.1 == 3)
//                     || (current_player_color == Black && target_pos.1 == 0))
//             {
//                 self.winner = Some(current_player_color);
//             }
//
//             // Switch turns.
//             self.is_white_turn = !self.is_white_turn;
//             Ok(())
//         } else {
//             // If the move is not valid, return an error.
//             Err(GameError::IllegalMove.into())
//         }
//     }
//
//     // Returns the color of the current turn.
//     pub fn get_turn(&self) -> Color {
//         if self.is_white_turn {
//             White
//         } else {
//             Black
//         }
//     }
//
//     // Drops a piece onto the board.
//     pub fn drop_piece(
//         &mut self,
//         piece: &mut Piece,
//         x: usize,
//         y: usize,
//     ) -> Result<(), Box<dyn Error>> {
//         // Check if the target position is occupied.
//         if self.get(x, y).is_some() {
//             Err(GameError::IllegalMove.into())
//         } else {
//             // Check if the piece is in the current player's cemetery.
//             let cemetary = match piece.color {
//                 White => &mut self.white_cemetery,
//                 Black => &mut self.black_cemetery,
//             };
//             if cemetary.contains(piece) {
//                 // If the piece is in the cemetery, remove it from the cemetery and place it on the board.
//                 cemetary.retain(|p| p != piece);
//                 if self.find_piece_on_board(piece).is_ok() {
//                     piece.is_duplicated = true;
//                 }
//                 self.put(x, y, piece);
//                 // Switch turns.
//                 self.is_white_turn = !self.is_white_turn;
//                 Ok(())
//             } else {
//                 // If the piece is not in the cemetery, return an error.
//                 // better error
//                 Err(GameError::IllegalMove.into())
//             }
//         }
//     }
//
//     // Checks if the game is over.
//     pub fn is_game_over(&self) -> bool {
//         self.winner.is_some()
//     }
//
//     // Prints the current state of the board and the cemeteries.
//     pub fn show(&self) {
//         println!("##############");
//
//         fn print_row_nb() {
//             print!("    0 1 2")
//         }
//
//         for y in (0..6).rev() {
//             match y {
//                 0 => print_row_nb(),
//                 5 => print_row_nb(),
//                 _ => self.a(y),
//             }
//
//             println!();
//         }
//         println!("##############\n");
//         println!(
//             "White cemetery: {:?}",
//             self.white_cemetery
//                 .iter()
//                 .map(|p| p.show().to_string())
//                 .collect::<Vec<String>>()
//         );
//         println!(
//             "Black cemetery: {:?}",
//             self.black_cemetery
//                 .iter()
//                 .map(|p| p.show().to_string())
//                 .collect::<Vec<String>>()
//         );
//     }
//
//     fn a(&self, y: usize) {
//         for x in 0..5 {
//             let y = y - 1;
//             match x {
//                 0 => print!("{} | ", y),
//                 4 => print!("| {}", y),
//                 _ => print!(
//                     "{} ",
//                     match self.get(x - 1, y) {
//                         Some(p) => p.show(),
//                         None => " ".to_string(),
//                     }
//                 ),
//             }
//         }
//     }
//
//     fn a2(&self, y: usize, s: &mut String) {
//         for x in 0..5 {
//             let y = y - 1;
//             match x {
//                 0 => s.push_str(&format!("{} | ", y)),
//                 4 => s.push_str(&format!("| {}", y)),
//                 _ => s.push_str(&format!(
//                     "{} ",
//                     match self.get(x - 1, y) {
//                         Some(p) => p.show(),
//                         None => " ".to_string(),
//                     }
//                 )),
//             }
//         }
//     }
//
//     pub fn get_hash(&self) -> String {
//         let mut s = String::new();
//         for y in 0..4 {
//             for x in 0..3 {
//                 match self.get(x, y) {
//                     Some(p) => {
//                         s.push_str(&format!("{}{}{}", p.show(), x, y));
//                     }
//                     None => s.push_str(&format!("#{}{}", x, y)),
//                 }
//             }
//         }
//         s
//     }
//
//     pub fn get_hash_optimized(&self) -> u128 {
//         let mut n: u128 = 0;
//         let mut count = 0;
//         for y in 0..4 {
//             for x in 0..3 {
//                 if let Some(p) = self.get(x, y) {
//                     let mut piece_value: u8 = match p.piece_type {
//                         Chick => 0,
//                         Giraffe => 2,
//                         Elephant => 4,
//                         Lion => 6,
//                         Hen => 8,
//                     };
//                     if p.color == Black {
//                         piece_value += 1;
//                     }
//                     n += piece_value as u128 * 10_u128.pow(count);
//                     count += 1;
//                     n += (x as u128) * 10_u128.pow(count);
//                     count += 1;
//                     n += (y as u128) * 10_u128.pow(count);
//                     count += 1;
//                 }
//             }
//         }
//         for p in self.white_cemetery.iter() {
//             let mut pt = match p.piece_type {
//                 Chick => 0,
//                 Giraffe => 2,
//                 Elephant => 4,
//                 Lion => 6,
//                 Hen => 8,
//             };
//             if p.color == Black {
//                 pt += 1;
//             }
//             n += pt as u128 * 10_u128.pow(count);
//             count += 1;
//             n += 9 * 10_u128.pow(count);
//             count += 1;
//             n += 9 * 10_u128.pow(count);
//             count += 1;
//         }
//         for p in self.black_cemetery.iter() {
//             let mut pt = match p.piece_type {
//                 Chick => 0,
//                 Giraffe => 2,
//                 Elephant => 4,
//                 Lion => 6,
//                 Hen => 8,
//             };
//             if p.color == Black {
//                 pt += 1;
//             }
//             n += pt as u128 * 10_u128.pow(count);
//             count += 1;
//             n += 8 * 10_u128.pow(count);
//             count += 1;
//             n += 8 * 10_u128.pow(count);
//             count += 1;
//         }
//         //println!("Hash: {}", n);
//         n
//     }
//
//     pub fn show_file(&self) -> String {
//         let mut s = String::new();
//         s.push_str(self.get_hash_optimized().to_string().as_str());
//         s.push_str("\n");
//         s.push_str("##############\n");
//
//         for y in (0..6).rev() {
//             match y {
//                 0 => s.push_str("    0 1 2"),
//                 5 => s.push_str("    0 1 2"),
//                 _ => self.a2(y, &mut s),
//             }
//
//             s.push_str("\n");
//         }
//         s.push_str("##############\n");
//         s.push_str(&format!(
//             "White cemetery: {:?}",
//             self.white_cemetery
//                 .iter()
//                 .map(|p| p.show().to_string())
//                 .collect::<Vec<String>>()
//         ));
//         s.push_str("\n");
//         s.push_str(&format!(
//             "Black cemetery: {:?}",
//             self.black_cemetery
//                 .iter()
//                 .map(|p| p.show().to_string())
//                 .collect::<Vec<String>>()
//         ));
//         s.push_str("\n\n");
//         s
//     }
//
//     pub fn next(&self) -> crate::structs::GameResult {
//         let mut boards = vec![];
//         for y in 0..4 {
//             for x in 0..3 {
//                 let p = self.get(x, y);
//                 match p {
//                     Some(p) => {
//                         if p.color != self.get_turn() {
//                             continue;
//                         }
//                         for m in self.get_legal_move_list_for_piece(&p).unwrap() {
//                             let mut b = self.clone();
//                             b.move_piece(&mut p.clone(), m).unwrap();
//                             if b.is_game_over() {
//                                 match b.winner {
//                                     Some(Color::White) => return WhiteWin,
//                                     Some(Color::Black) => return BlackWin,
//                                     _ => (),
//                                 }
//                             }
//                             boards.push(b.clone());
//                         }
//                     }
//                     None => {
//                         for p in self.get_curent_player_cemetery() {
//                             for y in 0..4 {
//                                 for x in 0..3 {
//                                     let mut b = self.clone();
//                                     if b.drop_piece(&mut p.clone(), x, y).is_ok() {
//                                         boards.push(b);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         Intermediate(boards)
//     }
// }
