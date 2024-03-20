use std::env;

use log::info;

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::piece::{Piece, HEN_2, LION_1};
use game_helper_v2::structs::GameResult;
use game_helper_v2::structs::Position::X0Y0;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let b = Board::init();

    //info!("{:?}", b.show_hex());
    //b.debug_show_board();

    let mut vec_b = vec![];

    b.debug_show_board_2();
    let a = b.get_next_states(true);
    match a {
        GameResult::WhiteWin => {
            info!("################################");
            info!("################################");
            info!("WW");
        }
        GameResult::BlackWin => {
            info!("################################");
            info!("################################");
            info!("BW");
        }
        GameResult::Intermediate(brds) => {
            info!("################################");
            info!("################################");
            info!("Intermediate");
            for b in brds {
                b.debug_show_board_2();
                vec_b.push(b);
            }
        }
    }

    println!("########################################");
    println!("############### DEPTH 2 ################");
    println!("########################################");

    for b in vec_b {
        let a = b.get_next_states(false);
        match a {
            GameResult::WhiteWin => {
                info!("################################");
                info!("################################");
                info!("WW");
            }
            GameResult::BlackWin => {
                info!("################################");
                info!("################################");
                info!("BW");
            }
            GameResult::Intermediate(brds) => {
                info!("################################");
                info!("################################");
                info!("Intermediate");
                for b in brds {
                    b.debug_show_board_2();
                }
            }
        }
    }
}
