use std::fmt::{Display, Formatter};
use piece::Color::{Black, White};
use piece::Piece;
use piece::Type::{Chick, Elephant, Giraffe, Lion};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    // 2d array of 3x4 pieces
    board: [[Option<Piece>; 3]; 4],
    white_cemetery: [Option<Piece>; 8],
    black_cemetery: [Option<Piece>; 8]
}

impl Board {
    pub fn new_empty() -> Board {
        Board{
            board: [[None; 3]; 4],
            white_cemetery: [None; 8],
            black_cemetery: [None; 8]
        }
    }

    // because of the way we store the board, we need to flip the y axis
    // this coordinate system is fucking with my head fr
    pub fn get(&self, x: i8, y: i8) -> Option<Piece> {
        self.board[y as usize][x as usize]
    }

    pub fn put(&mut self, x: i8, y: i8, p: Piece) -> &Board {
        self.board[y as usize][x as usize] = Some(p);
        self
    }

    pub fn del(&mut self, x: i8, y: i8) -> &Board {
        self.board[y as usize][x as usize] = None;
        self
    }

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