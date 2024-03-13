use std::env;

use log::info;

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::piece::{Piece, HEN_2, LION_1};
use game_helper_v2::structs::Position::X0Y0;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let b = Board::init();
    info!("{:?}", b.show_hex());
    b.get_at_pos_slow(X0Y0);
    b.debug_show_board();
    b.debug_show_board_2();
    //play_game();
    //board_generator::main();
    //board_generator_multi_threaded::main().await;
}
