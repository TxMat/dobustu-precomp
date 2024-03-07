use game_helper::board::Board;
use game_helper::piece::{Color, Piece, PieceType};

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

    let white_pieces = vec![c, g, e, l, h];
    let black_pieces = vec![C, G, E, L, H];

    let mut b = Board::init();
    while !b.is_game_over() {
        b.show();
        let random = rand::random::<usize>() % white_pieces.len();
        let random_m = rand::random::<usize>() % white_pieces[random].piece_type.moves().len();
        b.move_piece(
            &white_pieces[random],
            white_pieces[random].piece_type.moves()[random_m],
        );
    }
    b.show();
    println!("Game Over");
    match b.winner {
        Some(Color::White) => println!("White wins"),
        Some(Color::Black) => println!("Black wins"),
        None => println!("Draw"),
    }
}
