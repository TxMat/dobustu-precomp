use game_helper::board::Board;
use game_helper::moves::MOVE_N;
use game_helper::piece::{Piece, PieceType, Color};

fn main() {
    
    let c = Piece::new(PieceType::Chick, Color::White);
    let g = Piece::new(PieceType::Giraffe, Color::White);
    let e = Piece::new(PieceType::Elephant, Color::White);
    let l = Piece::new(PieceType::Lion, Color::White);
    let h = Piece::new(PieceType::Hen, Color::White);
    
    let C = Piece::new(PieceType::Chick, Color::Black);
    let G = Piece::new(PieceType::Giraffe, Color::Black);
    let E = Piece::new(PieceType::Elephant, Color::Black);
    let L = Piece::new(PieceType::Lion, Color::Black);
    let H = Piece::new(PieceType::Hen, Color::Black);
    
    let mut b = Board::init();
    b.show();
    b.move_piece(&c, MOVE_N);
    b.show();
}
