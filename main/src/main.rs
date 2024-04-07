use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::{env, mem, vec};

use log::{debug, error, info};

use game_helper_v2;
use game_helper_v2::board::Board;
use game_helper_v2::next_move::NextMove;
use game_helper_v2::piece::{
    CHICK_1, CHICK_2, ELEPHANT_1, ELEPHANT_2, GIRAFFE_1, GIRAFFE_2, LION_1, LION_2,
};
use game_helper_v2::structs::Calc::Proba;
use game_helper_v2::structs::Position::{
    Dead, X0Y0, X0Y1, X0Y2, X0Y3, X1Y0, X1Y1, X1Y2, X1Y3, X2Y0, X2Y1, X2Y3,
};
use game_helper_v2::structs::{Calc, GameResult};

fn main() {
    env::set_var("RUST_LOG", "debug");
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

    sequential_comp()

    // complete_black_comp(&mut file);
    // complete_white_comp(&mut file2);
    //bench();
    //calc();
}

// fn complete_black_comp(file: &mut File) {
//     let mut to_visit_b: VecDeque<Board> = std::collections::VecDeque::new();
//     let mut to_visit_w: VecDeque<Board> = std::collections::VecDeque::new();
//
//     let mut visited_player: HashMap<Board, (NextMove, f32)> = std::collections::HashMap::new();
//     let mut visited_w: HashSet<Board> = std::collections::HashSet::new();
//
//     let proba = recursive_comp(
//         true,
//         false,
//         0,
//         &mut to_visit_b,
//         &mut to_visit_w,
//         &mut visited_player,
//         &mut visited_w,
//         Board::init(),
//         file,
//     );
//
//     info!("Proba: {}", proba);
// }
//
// fn complete_white_comp(file: &mut File) {
//     let mut to_visit_b: VecDeque<Board> = std::collections::VecDeque::new();
//     let mut to_visit_w: VecDeque<Board> = std::collections::VecDeque::new();
//
//     let mut visited_player: HashMap<Board, (NextMove, f32)> = std::collections::HashMap::new();
//     let mut visited_w: HashSet<Board> = std::collections::HashSet::new();
//
//     let proba = recursive_comp(
//         true,
//         true,
//         0,
//         &mut to_visit_b,
//         &mut to_visit_w,
//         &mut visited_player,
//         &mut visited_w,
//         Board::init(),
//         file,
//     );
//
//     info!("Proba: {}", proba);
// }

const MAX_DEPTH: u8 = 11;

fn sequential_comp() {
    let mut calc_state: HashMap<u8, HashMap<Board, GameResult>> = HashMap::default();

    let mut is_player_one = true;

    let mut hs = HashMap::new();
    hs.insert(
        Board::init(),
        Board::init().get_next_states_2(is_player_one),
    );
    calc_state.insert(0, hs);

    info!("Generating...");

    for depth in 0u8..MAX_DEPTH {
        info!("Depth {}", depth);
        let current_hashmap = calc_state.get(&depth).unwrap();

        let mut next_hashmap = HashMap::new();
        for game_result in current_hashmap.values() {
            match game_result {
                GameResult::WhiteWin | GameResult::BlackWin => continue,
                GameResult::Intermediate(game_result_board_vec) => {
                    'outer: for (_, board) in game_result_board_vec {
                        if board.0 == 0x8C7C695547302A11 {
                            // error!("a");
                        }
                        if depth >= 2 {
                            for d in 0u8..depth - 1 {
                                if (is_player_one && d % 2 == 0) || (!is_player_one && d % 2 != 0) {
                                    continue;
                                }
                                if calc_state.get(&(d)).unwrap().contains_key(board) {
                                    continue 'outer;
                                }
                            }
                        }
                        if board.0 == 0x8C7C695547302A11 {
                            // error!("Found");
                        }
                        next_hashmap.insert(*board, board.get_next_states_2(!is_player_one));
                    }
                }
            }
        }
        calc_state.insert(depth + 1, next_hashmap);
        is_player_one = !is_player_one;
    }

    // duplicate_checker(&calc_state);

    info!("Calculating White");
    calc_proba(true, &calc_state);
    info!("Calculating Black");
    calc_proba(false, &calc_state);
}

fn duplicate_checker(calc_state: &HashMap<u8, HashMap<Board, GameResult>>) {
    let mut all_white: HashMap<&Board, &GameResult> = HashMap::default();
    let mut all_black: HashMap<&Board, &GameResult> = HashMap::default();
    let mut master_len_white = 0;
    let mut master_len_black = 0;

    for d in 0..MAX_DEPTH {
        if d % 2 == 0 {
            master_len_white += calc_state.get(&d).unwrap().len();
            all_white.extend(calc_state.get(&d).unwrap())
        } else {
            master_len_black += calc_state.get(&d).unwrap().len();
            all_black.extend(calc_state.get(&d).unwrap())
        }
    }

    debug!("White");
    info!("was : {}", master_len_white);
    info!("is : {}", all_white.len());

    debug!("Black");
    info!("was : {}", master_len_black);
    info!("is : {}", all_black.len());
}

fn calc_proba(is_player_one: bool, calc_state: &HashMap<u8, HashMap<Board, GameResult>>) {
    let probas_mine: Arc<RwLock<HashMap<Board, (f32, NextMove)>>> =
        Arc::new(RwLock::new(HashMap::default()));
    let probas_theirs: Arc<RwLock<HashMap<Board, (f32, NextMove)>>> =
        Arc::new(RwLock::new(HashMap::default()));
    let is_ours_to_play = if is_player_one {
        (MAX_DEPTH - 1) % 2 == 0
    } else {
        (MAX_DEPTH - 1) % 2 != 0
    };

    let list_ref = if is_ours_to_play {
        probas_mine.clone()
    } else {
        probas_theirs.clone()
    };

    for (b, game_result) in calc_state.get(&(MAX_DEPTH - 1)).unwrap() {
        let mut list_guard = list_ref.write().unwrap();
        match game_result {
            GameResult::WhiteWin => {
                let a;
                if let Some(old) = if is_player_one {
                    a = 1;
                    list_guard.insert(*b, (1f32, NextMove(0)))
                } else {
                    a = 0;
                    list_guard.insert(*b, (0f32, NextMove(0)))
                } {
                    error!("HASHMAP ERROR");
                    error!("{:X} already in probas WW", b.0);
                    error!("was {}, {:x}", old.0, old.1);
                    error!("is {}, {:x}", a, 0);
                }
            }

            GameResult::BlackWin => {
                let a;
                if let Some(old) = if is_player_one {
                    a = 0;
                    list_guard.insert(*b, (0f32, NextMove(0)))
                } else {
                    a = 1;
                    list_guard.insert(*b, (1f32, NextMove(0)))
                } {
                    error!("HASHMAP ERROR");
                    error!("{:X} already in probas BW", b.0);
                    error!("was {}, {:x}", old.0, old.1);
                    error!("is {}, {:x}", a, 0);
                }
            }
            GameResult::Intermediate(_) => {
                list_guard.insert(*b, (0.5f32, NextMove(0)));
            }
        }
    }

    for depth in (0..MAX_DEPTH - 1).rev() {
        info!("{}", depth);
        let is_our_turn = if is_player_one {
            depth % 2 == 0
        } else {
            depth % 2 != 0
        };

        let list_ref = if is_our_turn {
            probas_mine.clone()
        } else {
            probas_theirs.clone()
        };

        let list_ref_ennemy = if is_our_turn {
            probas_theirs.clone()
        } else {
            probas_mine.clone()
        };

        let current_hashmap: &HashMap<Board, GameResult> = calc_state.get(&depth).unwrap();

        for (b, game_result) in current_hashmap.iter() {
            let mut list_guard_mine = list_ref.write().unwrap();
            let probas_theirs = list_ref_ennemy.read().unwrap();
            match game_result {
                GameResult::WhiteWin => {
                    let a;
                    if let Some(old) = if is_player_one {
                        a = 1;
                        list_guard_mine.insert(*b, (1f32, NextMove(0)))
                    } else {
                        a = 0;
                        list_guard_mine.insert(*b, (0f32, NextMove(0)))
                    } {
                        // error!("{:X} already in probas WW", b.0);
                        // error!("was {}, {:x}", old.0, old.1);
                        // error!("is {}, {:x}", a, 0);
                    }
                }
                GameResult::BlackWin => {
                    let a;
                    if let Some(old) = if is_player_one {
                        a = 0;
                        list_guard_mine.insert(*b, (0f32, NextMove(0)))
                    } else {
                        a = 1;
                        list_guard_mine.insert(*b, (1f32, NextMove(0)))
                    } {
                        // error!("{:X} already in probas BW", b.0);
                        // error!("was {}, {:x}", old.0, old.1);
                        // error!("is {}, {:x}", a, 0);
                    }
                }
                GameResult::Intermediate(board_vec) => {
                    let mut best_move = (-2f32, NextMove(0));
                    if is_our_turn {
                        for (next, board) in board_vec {
                            if let Some(probas_tuple) = probas_theirs.get(board) {
                                if probas_tuple.0 > best_move.0 {
                                    best_move = (probas_tuple.0, *next)
                                }
                            } else if 0.5f32 > best_move.0 {
                                best_move = (0.5f32, *next)
                            }
                        }
                        if let Some(tuple) = list_guard_mine.insert(*b, best_move) {
                            // if tuple.0 != 0.5f32 {
                            //     error!("{:X} already in probas", b.0);
                            //     // ici
                            //     error!("was {}, {:x}", tuple.0, tuple.1);
                            //     error!("is {}, {:x}", best_move.0, best_move.1);
                            // }
                        }
                    } else {
                        let mut proba_sum = 0f32;
                        let mut board_number = 0f32;

                        for (_, board) in board_vec {
                            if let Some(probas_tuple) = probas_theirs.get(board) {
                                proba_sum += probas_tuple.0;
                                board_number += 1f32;
                            } else {
                                proba_sum += 0.5f32;
                                board_number += 1f32;
                            }
                        }

                        let final_proba = proba_sum / board_number;

                        if let Some(tuple) = list_guard_mine.insert(*b, (final_proba, NextMove(0)))
                        {
                            // if tuple.0 != 0.5f32 {
                            //     error!("{:X} already in probas ennemy", b.0);
                            //     // ici
                            //     error!("was {}, {:x}", tuple.0, tuple.1);
                            //     error!("is {}, {:x}", final_proba, best_move.1);
                            // }
                        }
                    }
                }
            }
        }
    }

    let mut f = if is_player_one {
        std::fs::File::create("white_probas_11_no0.txt").unwrap()
    } else {
        std::fs::File::create("black_probas_11_no0.txt").unwrap()
    };

    for (board, (_, next)) in probas_mine.read().unwrap().iter() {
        if next.0 != 0 {
            f.write_all(format!("{:X} {:X}\n", board.0, next).as_bytes())
                .unwrap();
        }
    }
}

// }

// fn recursive_comp(
//     is_white: bool,
//     is_mine: bool,
//     depth: u64,
//     to_visit_mine: &mut VecDeque<Board>,
//     to_visit_notmine: &mut VecDeque<Board>,
//     visited_player: &mut HashMap<Board, (NextMove, f32)>,
//     visited_ennemy: &mut HashSet<Board>,
//     current: Board,
//     file: &mut File,
// ) -> f32 {
//     // killswitch
//     if depth >= 11 {
//         return -1f32;
//     }
//
//     // debug
//     if visited_player.len() % 2_000_000 == 0 {
//         info!("visited mine: {}", visited_player.len());
//         info!("mine queue length: {}", to_visit_mine.len());
//         info!("their queue length: {}", to_visit_notmine.len());
//     }
//     if visited_ennemy.len() % 2_000_000 == 0 {
//         info!("visited not mine: {}", visited_ennemy.len());
//         info!("mine queue length: {}", to_visit_mine.len());
//         info!("their queue length: {}", to_visit_notmine.len());
//     }
//
//     // info!("Board at depth: {}", depth);
//     // current.debug_show_board_2();
//     match current.get_next_states_2(is_white) {
//         GameResult::WhiteWin => {
//             // info!("White wins");
//             if is_white && is_mine {
//                 1f32
//             } else {
//                 0f32
//             }
//         }
//         GameResult::BlackWin => {
//             // info!("Black wins");
//             if !is_white && is_mine {
//                 1f32
//             } else {
//                 0f32
//             }
//         }
//         GameResult::Intermediate(board_vec) => {
//             // populate the to_visit vecs
//
//             // info!("Intermediate for depth  {}", depth);
//
//             let mut to_visit_count = 0;
//             let mut moves: Vec<(Board, f32)> = vec![];
//
//             for b in board_vec {
//                 if is_mine {
//                     if let Some((_, p)) = visited_player.get(&b) {
//                         if *p >= 0f32 {
//                             moves.push((b, *p));
//                         }
//                         continue;
//                     }
//                     // info!("Ignored white board");
//                     // b.debug_show_board_2();
//                 } else {
//                     if visited_ennemy.contains(&b) {
//                         // info!("Ignored black board");
//                         // b.debug_show_board_2();
//                         continue;
//                     }
//                 }
//
//                 if to_visit_mine.contains(&b) {
//                     // info!("Ignored mine board");
//                     // b.debug_show_board_2();
//                     continue;
//                 }
//
//                 to_visit_mine.push_back(b);
//                 to_visit_count += 1;
//                 // info!("Intermediate white board");
//                 // b.debug_show_board_2();
//             }
//
//             let mut total_proba: f32 = 0f32;
//             let mut proba_number: f32 = 0f32;
//             for _ in 0..to_visit_count {
//                 let board = match to_visit_mine.pop_back() {
//                     Some(board) => {
//                         if is_mine {
//                             visited_ennemy.insert(board);
//                         } else {
//                             visited_player.insert(board, (NextMove(0), -1f32));
//                         }
//                         board
//                     }
//                     None => {
//                         // to visit empty
//                         return -10f32;
//                     }
//                 };
//
//                 let proba = recursive_comp(
//                     !is_white,
//                     !is_mine,
//                     depth + 1,
//                     to_visit_notmine, // swapped
//                     to_visit_mine,    // swapped
//                     visited_player,
//                     visited_ennemy,
//                     board,
//                     file,
//                 );
//
//                 if proba >= 0f32 {
//                     if is_mine {
//                         moves.push((board, proba));
//                     }
//                     total_proba += proba;
//                     proba_number += 1f32;
//                 }
//             }
//
//             if is_mine {
//                 let mut best_move = (Board::init(), -1f32);
//                 for move_ in moves {
//                     if move_.1 > best_move.1 {
//                         best_move.0 = move_.0;
//                         best_move.1 = move_.1;
//                     }
//                 }
//                 if best_move.1 < 0f32 {
//                     return -1f32;
//                 }
//                 let next_move = NextMove::new_from_board(&current, &best_move.0);
//                 // info!("visited player size before : {}", visited_player.len());
//                 // info!("is current in map : {}", visited_player.contains_key(&current));
//                 visited_player.insert(current, (next_move, best_move.1));
//                 // info!("visited player size after : {}", visited_player.len());
//                 file.write_all(format!("{:X} {:X}\n", current.0, next_move.0).as_bytes())
//                     .unwrap();
//
//                 // if depth < 4 {
//                 //     //
//                 //     info!("depth: {}, best move: {}", depth, best_move.1);
//                 //     info!("initial board");
//                 //     current.debug_show_board_2();
//                 //     info!("Best Next board");
//                 //     best_move.0.debug_show_board_2();
//                 // }
//             }
//
//             // debug if
//             if depth < 4 {
//                 if proba_number > 0f32 {
//                     info!("depth: {}, proba: {}", depth, total_proba / proba_number);
//                 }
//             }
//
//             if proba_number == 0f32 {
//                 return -1f32;
//             }
//             return total_proba / proba_number;
//         }
//     }
