use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::sync::Arc;

use log::info;
use tokio::sync::Mutex;

use game_helper::board::Board;
use game_helper::structs::GameResult;

pub(crate) async fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let stating_time = std::time::Instant::now();
    let mut last_time = stating_time.clone();

    let mut master_visited = Arc::new(Mutex::new(HashSet::new()));

    let mut boards = vec![Board::init()];
    let mut item_counts = [0, 0, 0];

    calc(
        999,
        boards[0].clone(),
        &mut boards,
        master_visited.clone(),
        &mut item_counts,
    )
    .await;

    let path = "boards.txt";
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);

    let mut handles = vec![];

    for thread in 0..boards.len() {
        let boards_clone = boards.clone();
        let visited_clone = master_visited.clone();

        let handle = tokio::spawn(async move {
            let b = boards_clone[thread].clone();
            let mut thread_board = vec![b];

            while let Some(b) = thread_board.pop() {
                calc(
                    thread,
                    b,
                    &mut thread_board,
                    visited_clone.clone(),
                    &mut item_counts,
                )
                .await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn calc(
    thread: usize,
    b: Board,
    boards: &mut Vec<Board>,
    visited: Arc<Mutex<HashSet<Board>>>,
    item_counts: &mut [i32; 3],
) {
    if visited.lock().await.contains(&b) {
        return;
    };

    match b.next() {
        GameResult::WhiteWin => item_counts[1] += 1,
        GameResult::BlackWin => item_counts[0] += 1,
        GameResult::Intermediate(bs) => {
            for b in bs {
                boards.push(b)
            }
            item_counts[2] += 1;
        }
    };

    let mut vg = visited.lock().await;

    vg.insert(b);
    //item_counts[(1 - r) as usize] += 1;
    if vg.len() % 1000000 == 0 {
        display_stats_simple(thread, item_counts);
    }
}

fn display_stats_simple(thread: usize, item_counts: &mut [i32; 3]) {
    info!(
        "enumerating... (WhiteWin: {}, BlackWin: {}, Intermediate: {}, total: {})",
        item_counts[0],
        item_counts[1],
        item_counts[2],
        item_counts[0] + item_counts[1] + item_counts[2]
    );
    info!("Thread: {}", thread);
}

// fn display_stats() {
//     let stat_time = last_time.elapsed().as_millis();
//     let total = item_counts[0] + item_counts[1] + item_counts[2];
//     info!(
//             "enumerating... (WhiteWin: {}, BlackWin: {}, Intermediate: {}, total: {})",
//             item_counts[0], item_counts[1], item_counts[2], total
//         );
//     info!(
//             "Generation progress: {}%, in {}s",
//             (total * 100) / 246803167,
//             stating_time.elapsed().as_secs()
//         );
//     info!(
//             "Current Generation Speed: {} Mboards/s",
//             2000000 / stat_time
//         );
//     info!(
//             "Average Generation Speed: {} Mboards/s",
//             total / stating_time.elapsed().as_millis()
//         );
//     info!(
//             "Estimated time to finish: {}s",
//             (246803167 - total) * stat_time / 1000 / 1000000
//         );
//     info!(
//             "Estimated time to finish: {}m",
//             (246803167 - total) * stat_time / 1000 / 1000000 / 60
//         );
//     info!("################################################################");
//     last_time = std::time::Instant::now();
// }
