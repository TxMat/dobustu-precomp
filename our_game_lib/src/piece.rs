use moves::{MOVE_CHICK, MOVE_ELEPHANT, MOVE_GIRAFFE, MOVE_HEN, MOVE_LION, MOVE_DUMMY};
use moves::Move;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    pub(crate) piece_type: Type,
    pub(crate) color: Color
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    White,
    Black
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Chick,
    Giraffe,
    Elephant,
    Lion,
    Hen
}

impl Piece {

    pub fn new(piece_type: Type, color: Color) -> Piece {
        Piece {
            piece_type,
            color
        }
    }

    pub fn moves(&self) -> &'static [&'static Move] {
        match self.piece_type {
            Type::Chick => MOVE_CHICK,
            Type::Giraffe => MOVE_GIRAFFE,
            Type::Elephant => MOVE_ELEPHANT,
            Type::Lion => MOVE_LION,
            Type::Hen => MOVE_HEN
        }
    }
    
    pub fn is_move_valid(&self, move_: Move) -> bool {
        self.moves().contains(&&move_)
    }

    pub fn show(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                Type::Chick => 'c',
                Type::Giraffe => 'g',
                Type::Elephant => 'e',
                Type::Lion => 'l',
                Type::Hen => 'h'
            },
            Color::Black => match self.piece_type {
                Type::Chick => 'C',
                Type::Giraffe => 'G',
                Type::Elephant => 'E',
                Type::Lion => 'L',
                Type::Hen => 'H'
            }
        }
    }
}