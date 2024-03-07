use game_helper::board::Board;
use game_helper::moves::MOVE_N;
use game_helper::piece::{Piece, Type, Color};

fn main() {
    let mut b = Board::init();
    b.show();
    b.move_piece(Piece::new(Type::Chick, Color::White), MOVE_N);
    b.show();
}
