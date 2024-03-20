use std::{env, mem, vec};

use log::info;

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::piece::{
    Piece, CHICK_1, CHICK_2, ELEPHANT_1, ELEPHANT_2, EMPTY, GIRAFFE_1, GIRAFFE_2, LION_1, LION_2,
};
use game_helper_v2::structs::GameResult;
use game_helper_v2::structs::Position::{
    Dead, X0Y0, X0Y1, X0Y2, X0Y3, X1Y0, X1Y1, X1Y2, X1Y3, X2Y0, X2Y1, X2Y3,
};

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let mut item_counts = vec![0, 0, 0];

    let mut board_vec = std::collections::VecDeque::new();
    let mut board_vec_temp: Vec<Board> = Vec::new();

    let mut visited_black = std::collections::HashSet::new();

    let state_1 = [
        (X1Y0, LION_1),
        (X1Y3, LION_2),
        (X0Y0, ELEPHANT_1),
        (X1Y2, ELEPHANT_2),
        (X2Y0, GIRAFFE_1),
        (X0Y3, GIRAFFE_2),
        (Dead, CHICK_1),
        (Dead, CHICK_2),
    ];

    let state_2 = [
        (X1Y0, LION_1),
        (X1Y3, LION_2),
        (X0Y0, ELEPHANT_1),
        (X2Y3, ELEPHANT_2),
        (X2Y1, GIRAFFE_1),
        (X0Y2, GIRAFFE_2),
        (X1Y1, CHICK_1),
        (X1Y2, CHICK_2),
    ];

    let state_3 = [
        (X2Y1, LION_1),
        (X1Y3, LION_2),
        (X0Y0, ELEPHANT_1),
        (X2Y3, ELEPHANT_2),
        (X2Y0, GIRAFFE_1),
        (X0Y2, GIRAFFE_2),
        (X1Y1, CHICK_1),
        (X1Y2, CHICK_2),
    ];

    let state_4 = [
        (X0Y1, LION_1),
        (X1Y3, LION_2),
        (X0Y0, ELEPHANT_1),
        (X2Y3, ELEPHANT_2),
        (X2Y0, GIRAFFE_1),
        (X0Y2, GIRAFFE_2),
        (X1Y1, CHICK_1),
        (X1Y2, CHICK_2),
    ];

    let mut b1 = Board::init();
    b1.put_state(state_1);

    let mut b2 = Board::init();
    b2.put_state(state_2);

    let mut b3 = Board::init();
    b3.put_state(state_3);

    let mut b4 = Board::init();
    b4.put_state(state_4);

    board_vec.push_back(b1);
    board_vec.push_back(b2);
    board_vec.push_back(b3);
    board_vec.push_back(b4);

    visited_black.insert(b1);
    visited_black.insert(b2);
    visited_black.insert(b3);
    visited_black.insert(b4);
    visited_black.insert(Board::init());

    // fuckery

    let b_init = Board::init();
    let vec_1 = match b_init.get_next_states(true) {
        GameResult::Intermediate(bs) => bs,
        _ => panic!("aaaaaaaaaa"),
    };

    let mut vec2 = vec![];

    for b in vec_1 {
        vec2.extend(match b.get_next_states(false) {
            GameResult::Intermediate(bs) => bs,
            _ => panic!("bbbbbbbbbbb"),
        })
    }

    visited_black.extend(vec2);

    let mut turn = true;

    let mut depth = 2;

    while !board_vec.is_empty() {
        while let Some(board) = board_vec.pop_front() {
            if !turn && visited_black.contains(&board) {
                continue;
            }

            let r = match board.get_next_states(turn) {
                GameResult::WhiteWin => 1,
                GameResult::BlackWin => 0,
                GameResult::Intermediate(bs) => {
                    for b in bs {
                        match turn {
                            true => {
                                board_vec_temp.push(b);
                            }
                            false => {
                                if !visited_black.contains(&b) {
                                    board_vec_temp.push(b);
                                }
                            }
                        }
                    }
                    -1
                }
            };

            item_counts[(1 - r) as usize] += 1;

            if !turn {
                visited_black.insert(board);
            }
        }

        let total = item_counts[0] + item_counts[1] + item_counts[2];
        info!(
            "enumerating... (White Win: {}, Black Win: {}, Intermediate: {}, total: {}, depth: {})",
            item_counts[0], item_counts[1], item_counts[2], total, depth
        );

        turn = !turn;
        //mem::swap(&mut board_vec, &mut board_vec_temp);
        board_vec = board_vec_temp.drain(..).collect();
        board_vec_temp.clear();
        depth += 1;
    }
}
