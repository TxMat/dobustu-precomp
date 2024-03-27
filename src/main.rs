use std::collections::{HashMap, HashSet, VecDeque};
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

use crate::range_set::RangeSet;

mod range_set;

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    complete_black_comp();
    //bench();
    //calc();
}

fn calc() {
    let mut item_counts = vec![0, 0, 0];

    let mut board_vec = std::collections::VecDeque::new();
    let mut board_vec_temp = std::collections::VecDeque::new();

    let mut visited_black = vec![];

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

    visited_black.push(b1);
    visited_black.push(b2);
    visited_black.push(b3);
    visited_black.push(b4);
    visited_black.push(Board::init());

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
                                board_vec_temp.push_back(b);
                            }
                            false => {
                                if !visited_black.contains(&b) {
                                    board_vec_temp.push_back(b);
                                }
                            }
                        }
                    }
                    -1
                }
            };

            item_counts[(1 - r) as usize] += 1;

            if !turn {
                visited_black.push(board);
            }
        }

        let total = item_counts[0] + item_counts[1] + item_counts[2];
        info!(
            "enumerating... (White Win: {}, Black Win: {}, Intermediate: {}, total: {}, depth: {})",
            item_counts[0], item_counts[1], item_counts[2], total, depth
        );

        turn = !turn;
        mem::swap(&mut board_vec, &mut board_vec_temp);
        //board_vec = board_vec_temp.drain(..).collect();
        board_vec_temp.clear();
        depth += 1;
    }
}

fn complete_black_comp() {
    let mut to_visit_b: VecDeque<Board> = std::collections::VecDeque::new();
    let mut to_visit_w: VecDeque<Board> = std::collections::VecDeque::new();

    let mut visited_b: HashSet<Board> = std::collections::HashSet::new();
    let mut visited_w: HashSet<Board> = std::collections::HashSet::new();

    let (w, t) = recursive_comp(
        true,
        false,
        0,
        &mut to_visit_b,
        &mut to_visit_w,
        &mut visited_b,
        &mut visited_w,
        Board::init(),
    );

    info!("Wins: {}, total: {}", w, t);
}

fn recursive_comp(
    is_white: bool,
    is_mine: bool,
    depth: u64,
    to_visit_b: &mut VecDeque<Board>,
    to_visit_w: &mut VecDeque<Board>,
    visited_b: &mut HashSet<Board>,
    visited_w: &mut HashSet<Board>,
    current: Board,
) -> (u64, u64) {
    if depth >= 10 {
        return (0, 0);
    }
    if visited_w.len() % 100_000 == 0 {
        info!("visited white: {}", visited_w.len());
    }
    if visited_b.len() % 100_000 == 0 {
        info!("visited black: {}", visited_b.len());
    }
    // info!("Board at depth: {}", depth);
    // current.debug_show_board_2();
    match current.get_next_states(is_white) {
        GameResult::WhiteWin => {
            // info!("White wins");
            if is_white && is_mine {
                (1, 1)
            } else {
                (0, 1)
            }
        }
        GameResult::BlackWin => {
            // info!("Black wins");
            if !is_white && is_mine {
                (1, 1)
            } else {
                (0, 1)
            }
        }
        GameResult::Intermediate(board_vec) => {
            // populate the to_visit vecs

            // info!("Intermediate for depth  {}", depth);

            let mut to_visit_count = 0;
            if is_white {
                for b in board_vec {
                    if visited_w.contains(&b) {
                        // info!("Ignored white board");
                        // b.debug_show_board_2();
                        continue;
                    }
                    to_visit_w.push_back(b);
                    to_visit_count += 1;
                    // info!("Intermediate white board");
                    // b.debug_show_board_2();
                }
            } else {
                for b in board_vec {
                    if visited_b.contains(&b) {
                        // info!("Ignored black board");
                        // b.debug_show_board_2();
                        continue;
                    }
                    to_visit_b.push_back(b);
                    to_visit_count += 1;
                    // info!("Intermediate black board");
                    // b.debug_show_board_2();
                }
            }

            let mut count_tuple = (0, 0);
            let mut moves = vec![];
            for _ in 0..to_visit_count {
                let (w, t);
                let board = if is_white {
                    match to_visit_w.pop_back() {
                        Some(board) => {
                            visited_w.insert(board);
                            board
                        }
                        None => {
                            return count_tuple;
                        }
                    }
                } else {
                    match to_visit_b.pop_back() {
                        Some(board) => {
                            visited_b.insert(board);
                            board
                        }
                        None => {
                            return count_tuple;
                        }
                    }
                };

                (w, t) = recursive_comp(
                    !is_white,
                    !is_mine,
                    depth + 1,
                    to_visit_b,
                    to_visit_w,
                    visited_b,
                    visited_w,
                    board.clone(),
                );

                if is_mine && t != 0 {
                    moves.push((board, w as f32 / t as f32));
                }

                // 0
                // 1 2 3 4
                // . . . . 5 5/15
                // . . . . 1 1/3
                // . . . . 2 . 4
                // . . . . 3 . 0
                // .......

                //
                // 1 1/3

                count_tuple.0 += w;
                count_tuple.1 += t;
            }
            if depth < 4 {
                if let Some(best_move) = moves.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                {
                    info!("initial board");
                    current.debug_show_board_2();
                    info!("Best Next board");
                    best_move.0.debug_show_board_2();
                }
                if count_tuple.1 > 0 {
                    info!(
                        "depth: {}, wins: {}, total: {}",
                        depth, count_tuple.0, count_tuple.1
                    );
                }
            }
            return count_tuple;
        }
    }
}

fn bench() {
    // store 100_000_000 boards in different collections to see who takes less ram
    // let mut boards = vec![];
    // let b = Board::init();
    // info!("starting...");
    // info!("Size of board: {}", std::mem::size_of_val(&b));
    // info!("Expected Size of vec board: {}", std::mem::size_of_val(&b) * 100_000_000);
    // info!("Size of vec before: {}", std::mem::size_of_val(&boards));
    // for i in 0..1_600_000_000 {
    //     boards.push(Board(i as u64));
    // }
    // info!("done");
    // info!("Size of vec after: {}", std::mem::size_of_val(&*boards));

    let mut boards = HashMap::new();
    let b = Board::init();
    info!("starting...");
    info!("Size of board: {}", std::mem::size_of_val(&b));
    info!(
        "Expected Size of vec board: {}",
        std::mem::size_of_val(&b) * 100_000_000
    );
    info!("Size of vec before: {}", std::mem::size_of_val(&boards));
    for i in 0..1_600_000_000 {
        boards.insert(Board(i as u64), Board(i as u64));
    }
    info!("done");
    info!("Size of vec after: {}", std::mem::size_of_val(&boards));

    //
    // let mut boards = std::collections::HashSet::new();
    // info!("starting...");
    // info!("Size of board: {}", std::mem::size_of_val(&b));
    // info!("Expected Size of vec board: {}", std::mem::size_of_val(&b) * 100_000_000);
    // info!("Size of vec before: {}", std::mem::size_of_val(&boards));
    // for i in 0..1_600_000_000 {
    //     boards.insert(Board(i as u64));
    // }
    // info!("done");
    // info!("Size of vec after: {}", std::mem::size_of_val(&boards));

    // let mut boards = std::collections::BTreeSet::new();
    // info!("starting...");
    // info!("Size of board: {}", std::mem::size_of_val(&b));
    // info!("Expected Size of vec board: {}", std::mem::size_of_val(&b) * 100_000_000);
    // info!("Size of vec before: {}", std::mem::size_of_val(&boards));
    // for i in 0..1_600_000_000 {
    //     boards.insert(Board(i as u64));
    // }
    // info!("done");
    // info!("Size of vec after: {}", std::mem::size_of_val(&boards));
}
