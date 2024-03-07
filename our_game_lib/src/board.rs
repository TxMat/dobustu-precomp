use std::vec;
// Importing the necessary modules and structs for the Board struct
use moves::Move;
use piece::Color::{Black, White};
use piece::{Color, Piece, Type};
use piece::Type::{Chick, Elephant, Giraffe, Lion};

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
    black_cemetery: Vec<Piece>
}

impl Board {
    /// Creates a new empty board.
    pub fn new_empty() -> Board {
        Board{
            board: [[None; 3]; 4],
            white_cemetery: Vec::new(),
            black_cemetery: Vec::new()
        }
    }

    /// Returns the piece at the given coordinates.
    /// The coordinates are flipped because of the way the board is stored.
    pub fn get(&self, x: i8, y: i8) -> Option<Piece> {
        self.board[y as usize][x as usize]
    }

    /// Places a piece at the given coordinates.
    pub fn put(&mut self, x: i8, y: i8, p: Piece) {
        self.board[y as usize][x as usize] = Some(p);
    }

    /// Removes the piece at the given coordinates.
    pub fn del(&mut self, x: i8, y: i8) {
        self.board[y as usize][x as usize] = None;
    }

    /// Initializes the board with the default setup.
    pub fn init() -> Board {
        let mut b = Board::new_empty();
        b.put(0, 0, Piece::new(Giraffe, White));
        b.put(1, 0, Piece::new(Lion, White));
        b.put(2, 0, Piece::new(Elephant, White));
        b.put(1, 1, Piece::new(Chick, White));
        b.put(1, 2, Piece::new(Chick, Black));
        b.put(2, 3, Piece::new(Elephant, Black));
        b.put(1, 3, Piece::new(Lion, Black));
        b.put(0, 3, Piece::new(Giraffe, Black));
        b
    }

    /// Returns a reference to a piece on the board if it exists.
    pub fn find_piece_on_board_opti(&self, piece: Piece) -> Option<&Piece> {
        // t'inquietes pas si tu ne comprends pas cette fonction
        self.board.iter().flatten()
        .find(|p| p.is_some() && p.unwrap() == piece)
        .map(|p| p.as_ref().unwrap())
    }

    pub fn find_piece_on_board(&self, piece: Piece) -> Option<(i8, i8)> {
        for y in 0..4 {
            for x in 0..3 {
                if self.get(x, y) == Some(piece) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Moves a piece on the board if the move is valid.
pub fn move_piece(&mut self, piece: Piece, move_: &Move) -> bool {
    if piece.is_move_valid(&move_) {
        let piece_pos = self.find_piece_on_board(piece).unwrap();
        let target_pos = (piece_pos.0 + move_.x, piece_pos.1 + move_.y);
        let captured_piece = self.get(target_pos.0, target_pos.1);

        self.del(piece_pos.0, piece_pos.1);
        if let Some(captured) = captured_piece {
            let cemetery = match captured.color {
                White => &mut self.white_cemetery,
                Black => &mut self.black_cemetery,
            };
            cemetery.push(captured);
        }
        self.put(target_pos.0, target_pos.1, piece);
        true
    } else {
        false
    }
}

    /// Prints the current state of the board and the cemeteries.
    pub fn show(&self) {
        println!("---");
        let mut s = String::new();
        for y in (0..4).rev() {
            for x in 0..3 {
                s.push(match self.get(x, y) {
                    Some(p) => p.show(),
                    None => ' '
                });
            }
            println!("{}", s);
            s.clear();
        }
        println!("---\n");
        println!("White cemetery: {:?}", self.white_cemetery);
        println!("Black cemetery: {:?}", self.black_cemetery);
    }
}