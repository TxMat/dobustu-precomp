use game_helper::board::Board;
use game_helper::piece::{Color, Piece, PieceType};

fn main() {
    let mut c = Piece::new(PieceType::Chick, Color::None);
    let mut g = Piece::new(PieceType::Giraffe, Color::None);
    let mut e = Piece::new(PieceType::Elephant, Color::None);
    let mut l = Piece::new(PieceType::Lion, Color::None);
    let mut h = Piece::new(PieceType::Hen, Color::None);

    let mut pieces = vec![c, g, e, l, h];

    let mut b = Board::init();
    b.show();
    while !b.is_game_over() {
        let random = rand::random::<usize>() % pieces.len();
        let random_m = rand::random::<usize>() % pieces[random].piece_type.moves().len();
        let move_ = pieces[random].piece_type.moves()[random_m];
        match b.move_piece(&mut pieces[random], move_) {
            Ok(_) => {
                println!("Move successful");
                b.show();
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Game Over");
    match b.winner {
        Some(Color::White) => println!("White wins"),
        Some(Color::Black) => println!("Black wins"),
        None => println!("Draw"),
        _ => println!("Unknown"),
    }
}
