use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Write;
use std::{env, mem, vec};

use log::info;

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::piece::{
    CHICK_1, CHICK_2, ELEPHANT_1, ELEPHANT_2, GIRAFFE_1, GIRAFFE_2, LION_1, LION_2,
};
use game_helper_v2::structs::Position::{
    Dead, X0Y0, X0Y1, X0Y2, X0Y3, X1Y0, X1Y1, X1Y2, X1Y3, X2Y0, X2Y1, X2Y3,
};
use game_helper_v2::structs::{GameResult, NextMove};

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    // write to a file
    let mut file = std::fs::File::create("output_b_d15.txt").unwrap();
    let mut file2 = std::fs::File::create("output_w_d15.txt").unwrap();

    let mut board_test = Board::new_empty();
    let state = [
        (LION_1, X1Y0),
        (LION_2, X1Y3),
        (ELEPHANT_1, X0Y0),
        (ELEPHANT_2, X2Y3),
        (GIRAFFE_1, X2Y0),
        (GIRAFFE_2, X0Y3),
        (CHICK_1, X1Y1),
        (CHICK_2, X1Y2),
    ];
    board_test.put_state(state);
    info!("board {:X}", board_test.0);
    board_test.debug_show_board_2();

    complete_black_comp(&mut file);
    complete_white_comp(&mut file2);
    //bench();
    //calc();
}

fn complete_black_comp(file: &mut File) {
    let mut to_visit_b: VecDeque<Board> = std::collections::VecDeque::new();
    let mut to_visit_w: VecDeque<Board> = std::collections::VecDeque::new();

    let mut visited_player: HashMap<Board, (NextMove, f32)> = std::collections::HashMap::new();
    let mut visited_w: HashSet<Board> = std::collections::HashSet::new();

    let proba = recursive_comp(
        true,
        false,
        0,
        &mut to_visit_b,
        &mut to_visit_w,
        &mut visited_player,
        &mut visited_w,
        Board::init(),
        file,
    );

    info!("Proba: {}", proba);
}

fn complete_white_comp(file: &mut File) {
    let mut to_visit_b: VecDeque<Board> = std::collections::VecDeque::new();
    let mut to_visit_w: VecDeque<Board> = std::collections::VecDeque::new();

    let mut visited_player: HashMap<Board, (NextMove, f32)> = std::collections::HashMap::new();
    let mut visited_w: HashSet<Board> = std::collections::HashSet::new();

    let proba = recursive_comp(
        true,
        true,
        0,
        &mut to_visit_b,
        &mut to_visit_w,
        &mut visited_player,
        &mut visited_w,
        Board::init(),
        file,
    );

    info!("Proba: {}", proba);
}

fn recursive_comp(
    is_white: bool,
    is_mine: bool,
    depth: u64,
    to_visit_mine: &mut VecDeque<Board>,
    to_visit_notmine: &mut VecDeque<Board>,
    visited_player: &mut HashMap<Board, (NextMove, f32)>,
    visited_ennemy: &mut HashSet<Board>,
    current: Board,
    file: &mut File,
) -> f32 {
    // killswitch
    if depth >= 15 {
        return -1f32;
    }

    // debug
    if visited_player.len() % 2_000_000 == 0 {
        info!("visited mine: {}", visited_player.len());
        info!("mine queue length: {}", to_visit_mine.len());
        info!("their queue length: {}", to_visit_notmine.len());
    }
    if visited_ennemy.len() % 2_000_000 == 0 {
        info!("visited not mine: {}", visited_ennemy.len());
        info!("mine queue length: {}", to_visit_mine.len());
        info!("their queue length: {}", to_visit_notmine.len());
    }

    // info!("Board at depth: {}", depth);
    // current.debug_show_board_2();
    match current.get_next_states(is_white) {
        GameResult::WhiteWin => {
            // info!("White wins");
            if is_white && is_mine {
                1f32
            } else {
                0f32
            }
        }
        GameResult::BlackWin => {
            // info!("Black wins");
            if !is_white && is_mine {
                1f32
            } else {
                0f32
            }
        }
        GameResult::Intermediate(board_vec) => {
            // populate the to_visit vecs

            // info!("Intermediate for depth  {}", depth);

            let mut to_visit_count = 0;
            let mut moves: Vec<(Board, f32)> = vec![];

            for b in board_vec {
                if is_mine {
                    if let Some((_, p)) = visited_player.get(&b) {
                        if *p >= 0f32 {
                            moves.push((b, *p));
                        }
                        continue;
                    }
                    // info!("Ignored white board");
                    // b.debug_show_board_2();
                } else {
                    if visited_ennemy.contains(&b) {
                        // info!("Ignored black board");
                        // b.debug_show_board_2();
                        continue;
                    }
                }

                if to_visit_mine.contains(&b) {
                    // info!("Ignored mine board");
                    // b.debug_show_board_2();
                    continue;
                }

                to_visit_mine.push_back(b);
                to_visit_count += 1;
                // info!("Intermediate white board");
                // b.debug_show_board_2();
            }

            let mut total_proba: f32 = 0f32;
            let mut proba_number: f32 = 0f32;
            for _ in 0..to_visit_count {
                let board = match to_visit_mine.pop_back() {
                    Some(board) => {
                        if is_mine {
                            visited_ennemy.insert(board);
                        } else {
                            visited_player.insert(board, (NextMove(0), -1f32));
                        }
                        board
                    }
                    None => {
                        // to visit empty
                        return -10f32;
                    }
                };

                let proba = recursive_comp(
                    !is_white,
                    !is_mine,
                    depth + 1,
                    to_visit_notmine, // swapped
                    to_visit_mine,    // swapped
                    visited_player,
                    visited_ennemy,
                    board,
                    file,
                );

                if proba >= 0f32 {
                    if is_mine {
                        moves.push((board, proba));
                    }
                    total_proba += proba;
                    proba_number += 1f32;
                }
            }

            if is_mine {
                let mut best_move = (Board::init(), -1f32);
                for move_ in moves {
                    if move_.1 > best_move.1 {
                        best_move.0 = move_.0;
                        best_move.1 = move_.1;
                    }
                }
                if best_move.1 < 0f32 {
                    return -1f32;
                }
                let next_move = NextMove::new_from_board(&current, &best_move.0);
                // info!("visited player size before : {}", visited_player.len());
                // info!("is current in map : {}", visited_player.contains_key(&current));
                visited_player.insert(current, (next_move.clone(), best_move.1));
                // info!("visited player size after : {}", visited_player.len());
                file.write_all(format!("{:X} {:X}\n", current.0, next_move.0).as_bytes())
                    .unwrap();

                // if depth < 4 {
                //     //
                //     info!("depth: {}, best move: {}", depth, best_move.1);
                //     info!("initial board");
                //     current.debug_show_board_2();
                //     info!("Best Next board");
                //     best_move.0.debug_show_board_2();
                // }
            }

            // debug if
            if depth < 4 {
                if proba_number > 0f32 {
                    info!("depth: {}, proba: {}", depth, total_proba / proba_number);
                }
            }

            if proba_number == 0f32 {
                return -1f32;
            }
            return total_proba / proba_number;
        }
    }
}

fn complete_black_comp_omg(file: &mut File) {
    let mut stack: Vec<(bool, bool, u64, Board)> = Vec::new();
    let mut visited_player: HashMap<Board, (NextMove, f32)> = HashMap::new();
    let mut visited_ennemy: HashSet<Board> = HashSet::new();

    stack.push((true, false, 0, Board::init()));

    while let Some((is_white, is_mine, depth, current)) = stack.pop() {
        if visited_player.len() % 2_000_000 == 0 {
            info!("visited mine: {}", visited_player.len());
        }
        if visited_ennemy.len() % 2_000_000 == 0 {
            info!("visited not mine: {}", visited_ennemy.len());
        }

        match current.get_next_states(is_white) {
            GameResult::WhiteWin => {
                if is_white && is_mine {
                    visited_player.insert(current, (NextMove(0), 1f32));
                }
            }
            GameResult::BlackWin => {
                if !is_white && is_mine {
                    visited_player.insert(current, (NextMove(0), 1f32));
                }
            }
            GameResult::Intermediate(board_vec) => {
                for b in board_vec {
                    if is_mine {
                        if visited_player.get(&b).is_some() {
                            continue;
                        }
                        visited_ennemy.insert(b);
                    } else {
                        if visited_ennemy.contains(&b) {
                            continue;
                        }
                        visited_player.insert(b, (NextMove(0), -1f32));
                    }
                    stack.push((!is_white, !is_mine, depth + 1, b));
                }
            }
        }
    }

    let proba: f32 = visited_player.values().map(|&(_, p)| p).sum();
    info!("Proba: {}", proba);
}
