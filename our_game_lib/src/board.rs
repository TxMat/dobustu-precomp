use std::error::Error;

// Importing the necessary modules and structs for the Board struct
use moves::Move;
use piece::Color::{Black, White};
use piece::Piece;
use piece::PieceType::{Chick, Elephant, Giraffe, Lion};
use piece::{Color, PieceType};
use structs::GameError;

/// Represents the game board.
/// The board is a 2D array of 3x4 pieces.
/// Each piece can be either White or Black.
/// The board also includes a cemetery for each color to store the captured pieces.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // 2D array of 3x4 pieces
    board: [[Option<Piece>; 3]; 4],
    // Cemetery for the white pieces
    white_cemetery: Vec<Piece>,
    // Cemetery for the black pieces
    black_cemetery: Vec<Piece>,
    is_white_turn: bool,
    pub winner: Option<Color>,
}

impl Board {
    /// Creates a new empty board.
    pub fn new_empty() -> Board {
        Board {
            board: [[None; 3]; 4],
            white_cemetery: Vec::new(),
            black_cemetery: Vec::new(),
            is_white_turn: true,
            winner: None,
        }
    }

    /// Returns the piece at the given coordinates.
    /// The coordinates are flipped because of the way the board is stored.
    pub fn get(&self, x: usize, y: usize) -> Option<Piece> {
        self.board[y][x]
    }

    /// Places a piece at the given coordinates.
    pub fn put(&mut self, x: usize, y: usize, p: &Piece) {
        self.board[y][x] = Some(*p);
    }

    /// Removes the piece at the given coordinates.
    pub fn del(&mut self, x: usize, y: usize) {
        self.board[y][x] = None;
    }

    /// Initializes the board with the default setup.
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

    /// Returns a reference to a piece on the board if it exists.
    pub fn find_piece_on_board_opti(&self, piece: Piece) -> Option<&Piece> {
        // t'inquietes pas si tu ne comprends pas cette fonction
        self.board
            .iter()
            .flatten()
            .find(|p| p.is_some() && p.unwrap() == piece)
            .map(|p| p.as_ref().unwrap())
    }

    pub fn find_piece_on_board(&self, piece: &Piece) -> Option<(usize, usize)> {
        for y in 0..4 {
            for x in 0..3 {
                if self.get(x, y) == Some(*piece) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Moves a piece on the board if the move is valid.
    pub fn move_piece(&mut self, piece: &mut Piece, move_: &Move) -> Result<(), Box<dyn Error>> {
        if piece.is_move_valid(&move_) {
            piece.color = self.get_turn();
            let piece_pos = self
                .find_piece_on_board(&piece)
                .ok_or::<GameError>(GameError::PieceNotInBoard.into())?;

            let (x, y) = (piece_pos.0 as i8 + move_.x, piece_pos.1 as i8 + move_.y);
            if x > 2 || y > 3 || x < 0 || y < 0 {
                return Err(GameError::OutOfBounds.into());
            }

            let current_player_color = self.get_turn();

            let target_pos = (x as usize, y as usize);
            let piece_on_the_way = self.get(target_pos.0, target_pos.1);

            if let Some(p) = piece_on_the_way {
                if p.color == current_player_color {
                    return Err(GameError::IllegalMove.into());
                }
                if p.piece_type == Lion {
                    self.winner = Some(current_player_color);
                }
                let cemetery = match current_player_color {
                    White => &mut self.white_cemetery,
                    Black => &mut self.black_cemetery,
                    _ => unreachable!(),
                };
                cemetery.push(p);
            }

            self.del(piece_pos.0, piece_pos.1);

            // if the chick reaches the last row, it becomes a hen
            if piece.piece_type == Chick && (target_pos.1 == 0 || target_pos.1 == 3) {
                self.put(
                    target_pos.0,
                    target_pos.1,
                    &Piece::new(PieceType::Hen, current_player_color),
                );
            } else {
                self.put(target_pos.0, target_pos.1, &piece);
            }

            // if lion is on the last row, the game is over
            if piece.piece_type == Lion {
                if (current_player_color == White && target_pos.1 == 3)
                    || (current_player_color == Black && target_pos.1 == 0)
                {
                    self.winner = Some(current_player_color);
                }
            }

            self.is_white_turn = !self.is_white_turn;
            Ok(())
        } else {
            Err(GameError::IllegalMove.into())
        }
    }

    pub fn get_turn(&self) -> Color {
        if self.is_white_turn {
            White
        } else {
            Black
        }
    }

    pub fn drop_piece(
        &mut self,
        piece: &Piece,
        color: Color,
        x: usize,
        y: usize,
    ) -> Result<(), Box<dyn Error>> {
        if self.get(x, y).is_some() {
            return Err(GameError::IllegalMove.into());
        } else {
            let cemetary = match color {
                White => &mut self.white_cemetery,
                Black => &mut self.black_cemetery,
                _ => unreachable!(),
            };
            if cemetary.contains(piece) {
                cemetary.retain(|p| p != piece);
                self.put(x, y, piece);
                self.is_white_turn = !self.is_white_turn;
                Ok(())
            } else {
                Err(GameError::IllegalMove.into())
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }

    /// Prints the current state of the board and the cemeteries.
    pub fn show(&self) {
        println!("---");
        let mut s = String::new();
        for y in (0..4).rev() {
            for x in 0..3 {
                s.push(match self.get(x, y) {
                    Some(p) => p.show(),
                    None => ' ',
                });
            }
            println!("{}", s);
            s.clear();
        }
        println!("---\n");
        let white_cemetery: Vec<String> = self
            .white_cemetery
            .iter()
            .map(|p| p.show().to_string())
            .collect();
        let black_cemetery: Vec<String> = self
            .black_cemetery
            .iter()
            .map(|p| p.show().to_string())
            .collect();
        println!("White cemetery: {:?}", white_cemetery);
        println!("Black cemetery: {:?}", black_cemetery);
    }
}
