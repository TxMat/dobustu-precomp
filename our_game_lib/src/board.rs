use std::error::Error;
// Importing the necessary modules and structs for the Board struct
use moves::Move;
use piece::Color::{Black, White};
use piece::Piece;
use piece::PieceType::{Chick, Elephant, Giraffe, Lion};
use piece::{Color, PieceType};
use structs::GameError;

// The `derive` attribute automatically implements the specified traits for the struct.
// Clone: Allows the struct to be duplicated.
// PartialEq and Eq: Allows the struct to be compared for equality.
// Hash: Allows the struct to be used as a key in a HashMap.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // 2D array of 3x4 pieces. `Option` is a Rust enum that can either be `Some` or `None`.
    // `Some` indicates that there is a value and `None` indicates that there is no value.
    board: [[Option<Piece>; 3]; 4],
    // Cemetery for the white pieces. `Vec` is a growable, heap-allocated data structure.
    white_cemetery: Vec<Piece>,
    // Cemetery for the black pieces.
    black_cemetery: Vec<Piece>,
    // Boolean to keep track of whose turn it is.
    is_white_turn: bool,
    // The winner of the game, if there is one.
    pub winner: Option<Color>,
}

impl Board {
    // Creates a new empty board.
    pub fn new_empty() -> Board {
        Board {
            board: [[None; 3]; 4],
            white_cemetery: Vec::new(),
            black_cemetery: Vec::new(),
            is_white_turn: true,
            winner: None,
        }
    }

    // Returns the piece at the given coordinates.
    // The coordinates are flipped because of the way the board is stored.
    pub fn get(&self, x: usize, y: usize) -> Option<Piece> {
        self.board[y][x]
    }

    // Places a piece at the given coordinates.
    pub fn put(&mut self, x: usize, y: usize, p: &Piece) {
        self.board[y][x] = Some(*p);
    }

    // Removes the piece at the given coordinates.
    pub fn del(&mut self, x: usize, y: usize) {
        self.board[y][x] = None;
    }

    // Initializes the board with the default setup.
    pub fn init() -> Board {
        let mut b = Board::new_empty();
        b.put(0, 0, &Piece::new(Giraffe, White));
        b.put(1, 0, &Piece::new(Lion, White));
        b.put(2, 0, &Piece::new(Elephant, White));
        b.put(1, 1, &Piece::new(Chick, White));
        b.put(1, 2, &Piece::new(Chick, Black));
        b.put(2, 3, &Piece::new(Elephant, Black));
        b.put(1, 3, &Piece::new(Lion, Black));
        b.put(0, 3, &Piece::new(Giraffe, Black));
        b
    }

    // Finds a piece on the board and returns its coordinates.
    pub fn find_piece_on_board(&self, piece: &Piece) -> Result<(usize, usize), Box<dyn Error>> {
        for y in 0..4 {
            for x in 0..3 {
                if self.get(x, y) == Some(*piece) {
                    return Ok((x, y));
                }
            }
        }
        Err(GameError::PieceNotInBoard.into())
    }

    // Moves a piece on the board if the move is valid.
    // Returns a `Result` type which is an enum that can either be `Ok` or `Err`.
    // `Ok` indicates a successful operation and `Err` indicates an error.
    pub fn move_piece(&mut self, piece: &mut Piece, move_: &Move) -> Result<(), Box<dyn Error>> {
        // Check if the move is valid for the piece.
        if piece.is_move_valid(&move_) {
            // Set the color of the piece to the current turn's color.
            piece.color = self.get_turn();
            // Find the piece on the board.
            let piece_pos = self.find_piece_on_board(&piece)?;

            // Calculate the new coordinates for the piece.
            let (x, y) = (piece_pos.0 as i8 + move_.x, piece_pos.1 as i8 + move_.y);
            // Check if the new coordinates are out of bounds.
            if x > 2 || y > 3 || x < 0 || y < 0 {
                return Err(GameError::OutOfBounds.into());
            }

            // Get the color of the current player.
            let current_player_color = self.get_turn();

            // Get the piece at the target position.
            let target_pos = (x as usize, y as usize);
            let piece_on_the_way = self.get(target_pos.0, target_pos.1);

            // If there is a piece at the target position, check if it's the same color as the current player.
            if let Some(p) = piece_on_the_way {
                if p.color == current_player_color {
                    return Err(GameError::IllegalMove.into());
                }
                // If the piece at the target position is a Lion, the current player wins.
                if p.piece_type == Lion {
                    self.winner = Some(current_player_color);
                }
                // Add the piece at the target position to the current player's cemetery.
                let cemetery = match current_player_color {
                    White => &mut self.white_cemetery,
                    Black => &mut self.black_cemetery,
                    _ => unreachable!(),
                };
                cemetery.push(p);
            }

            // Remove the piece from its current position.
            self.del(piece_pos.0, piece_pos.1);

            // If the piece is a Chick and it reaches the last row, it becomes a Hen.
            if piece.piece_type == Chick && (target_pos.1 == 0 || target_pos.1 == 3) {
                self.put(
                    target_pos.0,
                    target_pos.1,
                    &Piece::new(PieceType::Hen, current_player_color),
                );
            } else {
                // Otherwise, move the piece to the target position.
                self.put(target_pos.0, target_pos.1, &piece);
            }

            // If the piece is a Lion and it reaches the last row, the game is over.
            if piece.piece_type == Lion {
                if (current_player_color == White && target_pos.1 == 3)
                    || (current_player_color == Black && target_pos.1 == 0)
                {
                    self.winner = Some(current_player_color);
                }
            }

            // Switch turns.
            self.is_white_turn = !self.is_white_turn;
            Ok(())
        } else {
            // If the move is not valid, return an error.
            Err(GameError::IllegalMove.into())
        }
    }

    // Returns the color of the current turn.
    pub fn get_turn(&self) -> Color {
        if self.is_white_turn {
            White
        } else {
            Black
        }
    }

    // Drops a piece onto the board.
    pub fn drop_piece(
        &mut self,
        piece: &Piece,
        color: Color,
        x: usize,
        y: usize,
    ) -> Result<(), Box<dyn Error>> {
        // Check if the target position is occupied.
        if self.get(x, y).is_some() {
            return Err(GameError::IllegalMove.into());
        } else {
            // Check if the piece is in the current player's cemetery.
            let cemetary = match color {
                White => &mut self.white_cemetery,
                Black => &mut self.black_cemetery,
                _ => unreachable!(),
            };
            if cemetary.contains(piece) {
                // If the piece is in the cemetery, remove it from the cemetery and place it on the board.
                cemetary.retain(|p| p != piece);
                self.put(x, y, piece);
                // Switch turns.
                self.is_white_turn = !self.is_white_turn;
                Ok(())
            } else {
                // If the piece is not in the cemetery, return an error.
                Err(GameError::IllegalMove.into())
            }
        }
    }

    // Checks if the game is over.
    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }

    // Prints the current state of the board and the cemeteries.
    pub fn show(&self) {
        println!("---");
        for y in (0..4).rev() {
            for x in 0..3 {
                print!(
                    "{}",
                    match self.get(x, y) {
                        Some(p) => p.show(),
                        None => ' ',
                    }
                );
            }
            println!();
        }
        println!("---\n");
        println!(
            "White cemetery: {:?}",
            self.white_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
        println!(
            "Black cemetery: {:?}",
            self.black_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
    }
}
