use std::{env, vec};

use log::info;

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::piece::{Piece, HEN_2, LION_1};
use game_helper_v2::structs::GameResult;
use game_helper_v2::structs::Position::X0Y0;

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut item_counts = vec![0, 0, 0];

    let mut board_vec = vec![Board::init()];
    let mut board_vec_temp: Vec<Board> = vec![];

    let mut visited_white = std::collections::HashSet::new();
    let mut visited_black = std::collections::HashSet::new();

    let mut turn = true;

    let mut depth = 0;

    //info!("{:?}", b.show_hex());
    //b.debug_show_board();

    while !board_vec.is_empty() {
        depth += 1;
        let visited = match turn {
            true => &mut visited_white,
            false => &mut visited_black,
        };
        for board in board_vec {
            if visited.contains(&board) {
                continue;
            }

            let r = match board.get_next_states(turn) {
                GameResult::WhiteWin => 1,
                GameResult::BlackWin => 0,
                GameResult::Intermediate(bs) => {
                    for b in bs {
                        board_vec_temp.push(b)
                    }
                    -1
                }
            };

            item_counts[(1 - r) as usize] += 1;

            visited.insert(board);
        }

        let total = item_counts[0] + item_counts[1] + item_counts[2];
        info!(
            "enumerating... (White Win: {}, Black Win: {}, Intermediate: {}, total: {}, depth: {})",
            item_counts[0], item_counts[1], item_counts[2], total, depth
        );

        turn = !turn;
        board_vec = board_vec_temp.clone();
        board_vec_temp.clear();
    }

    // while let Some(board) = board_vec.pop() {
    //     let visited = match turn {
    //         true => &mut visited_white,
    //         false => &mut visited_black,
    //     };
    //     if visited.contains(&board) {
    //         continue;
    //     }
    //
    //     let r = match board.get_next_states(turn) {
    //         GameResult::WhiteWin => {
    //             1
    //         },
    //         GameResult::BlackWin => {
    //             0
    //         },
    //         GameResult::Intermediate(bs) => {
    //             for b in bs {
    //                 board_vec.push(b)
    //             }
    //             -1
    //         }
    //     };
    //
    //     item_counts[(1 - r) as usize] += 1;
    //     if visited.len() % 1000000 == 0 {
    //         let total = item_counts[0] + item_counts[1] + item_counts[2];
    //         info!("enumerating... (White Win: {}, Black Win: {}, Intermediate: {}, total: {})",
    //             item_counts[0], item_counts[1], item_counts[2], total);
    //     }
    //
    //     visited.insert(board);
    //     turn = !turn;
    // }
}
