use std::env;
use std::error::Error;
use std::io::Write;

use log::{error, info, warn};

use game_helper::board::Board;
use game_helper::moves::{MOVE_E, MOVE_N, MOVE_NE, MOVE_NW, MOVE_S, MOVE_SE, MOVE_SW, MOVE_W};
use game_helper::piece::{Color, Piece, PieceType};
use game_helper::structs::GameError::{CantMoveAnywhere, EmptyCemetary, InavlidPiece, InvalidMove};

fn main() {
    let mut b = Board::init();

    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    while !b.is_game_over() {
        // clear the terminal output
        print!("\x1B[2J\x1B[1;1H");
        flush();

        warn!("{:?}'s turn", b.get_turn());
        b.show();
        info!("Enter piece to move or write 'drop' to drop one from your cemetery: ");

        let available_pieces = b.get_curent_player_piece_list();
        for p in &available_pieces {
            print!("{}, ", p.show());
        }
        print!("\n\n");
        flush();

        match choose_piece(&b) {
            Ok(mut piece) => {
                if let Err(e) = move_piece(&mut b, &mut piece) {
                    error!("Invalid move: {}", e);
                    wait_for_input();
                }
            }
            Err(e) => {
                if e != "drop" {
                    error!("Invalid piece: {}", e);
                    wait_for_input();
                    continue;
                }
                if let Err(e) = drop_piece(&mut b) {
                    error!("Invalid drop: {}", e);
                    wait_for_input();
                }
            }
        }
    }
    game_over(&b);
}

fn flush() {
    std::io::stdout().flush().unwrap();
}

fn wait_for_input() {
    let mut input = String::new();
    info!("Press enter to continue...");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn choose_piece(board: &Board) -> Result<Piece, String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "c" => Ok(Piece::new(PieceType::Chick, board.get_turn())),
        "g" => Ok(Piece::new(PieceType::Giraffe, board.get_turn())),
        "e" => Ok(Piece::new(PieceType::Elephant, board.get_turn())),
        "l" => Ok(Piece::new(PieceType::Lion, board.get_turn())),
        "h" => Ok(Piece::new(PieceType::Hen, board.get_turn())),
        _ => Err(input.trim().to_lowercase()),
    }
}

fn drop_piece(b: &mut Board) -> Result<(), Box<dyn Error>> {
    let available_pieces = b.get_curent_player_cemetery();

    if available_pieces.is_empty() {
        return Err(EmptyCemetary.into());
    }

    info!("Enter piece to drop: ");
    for p in &available_pieces {
        print!("{}, ", p.show());
    }
    print!("\n\n");
    std::io::stdout().flush().unwrap();

    match choose_piece(&b) {
        Ok(p) => drop_piece_coors(b, &p),
        Err(_) => Err(InavlidPiece.into()),
    }
}

fn drop_piece_coors(b: &mut Board, piece: &Piece) -> Result<(), Box<dyn Error>> {
    info!("Enter coors to drop piece: (without spaces)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let coors: usize = input.trim().parse()?;

    let x = coors / 10;
    let y = coors % 10;

    b.drop_piece(&piece, x, y)
}

fn move_piece(b: &mut Board, piece: &mut Piece) -> Result<(), Box<dyn Error>> {
    println!("Enter move type: ");

    let m = b.get_legal_move_list_for_piece(&piece)?;

    if m.is_empty() {
        return Err(CantMoveAnywhere.into());
    }

    for m in m.iter() {
        print!("{}, ", m);
    }

    print!("\n\n");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let move_type = match input.trim().to_lowercase().as_str() {
        "n" => MOVE_N,
        "ne" => MOVE_NE,
        "e" => MOVE_E,
        "se" => MOVE_SE,
        "s" => MOVE_S,
        "sw" => MOVE_SW,
        "w" => MOVE_W,
        "nw" => MOVE_NW,
        _ => return Err(InvalidMove.into()),
    };

    b.move_piece(piece, move_type)
}

fn game_over(b: &Board) {
    error!("Game Over");
    match b.winner {
        Some(Color::White) => println!("White wins"),
        Some(Color::Black) => println!("Black wins"),
        _ => unreachable!("Game over but no winner"),
    }
}
