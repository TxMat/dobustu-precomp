use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Write;
use std::{env, mem, vec};

use log::info;

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

const MAX_DEPTH: u8 = 5;

fn sequential_comp() {
    let mut calc_state: HashMap<u8, &mut HashMap<Board, Calc<GameResult>>> = HashMap::default();

    let mut is_player_one = true;

    let mut hs = HashMap::new();
    hs.insert(
        Board::init(),
        Calc::GameResult(Board::init().get_next_states_2(is_player_one)),
    );
    calc_state.insert(0, &mut hs);

    for depth in 0u8..MAX_DEPTH {
        let current_hashmap = calc_state.get(&depth).unwrap();

        let mut next_hashmap = HashMap::new();
        for game_result in current_hashmap.values() {
            match game_result.unwrap_some() {
                GameResult::WhiteWin | GameResult::BlackWin => continue,
                GameResult::Intermediate(game_result_board_vec) => {
                    'outer: for (_, board) in game_result_board_vec {
                        if depth >= 2 {
                            for d in 0u8..depth - 1 {
                                if (!is_player_one && d % 2 == 0) || (is_player_one && d % 2 != 0) {
                                    continue;
                                }
                                if calc_state.get(&d).unwrap().contains_key(board) {
                                    continue 'outer;
                                }
                            }
                        }
                        next_hashmap.insert(
                            *board,
                            Calc::GameResult(board.get_next_states_2(!is_player_one)),
                        );
                    }
                }
            }
        }
        calc_state.insert(depth + 1, &mut next_hashmap);
        is_player_one = !is_player_one;
    }

    calc_proba(true, &mut calc_state);
    // calc_proba(false, &calc_state);
}

fn calc_proba(
    is_player_one: bool,
    calc_state: &mut HashMap<u8, &mut HashMap<Board, Calc<GameResult>>>,
) {
    for depth in (0..crate::MAX_DEPTH).rev() {
        let current_hashmap: &mut HashMap<Board, Calc<game_helper_v2::structs::GameResult>> =
            *calc_state.get(&depth).unwrap();
        let next_hashmap = calc_state.get(&(depth + 1)).unwrap();
        for (b, calc) in current_hashmap.iter_mut() {
            match calc.unwrap_some() {
                GameResult::WhiteWin => {
                    if is_player_one {
                        *calc = Proba((NextMove(0), 1f32));
                    } else {
                        *calc = Proba((NextMove(0), 0f32));
                    }
                }
                GameResult::BlackWin => {
                    if is_player_one {
                        *calc = Proba((NextMove(0), 0f32));
                    } else {
                        *calc = Proba((NextMove(0), 1f32));
                    }
                }
                GameResult::Intermediate(board_vec) => for (next, board) in board_vec {},
            }
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
