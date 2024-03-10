use std::char::from_digit;
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

    let mut boards = vec![Board::init()];

    let master_visited = Arc::new(Mutex::new(HashSet::new()));
    let master_item_counts = Arc::new(Mutex::new([0, 0, 0]));

    let path = "boards_init.txt";
    let f = File::create(path).expect("unable to create file");
    let mut f = Arc::new(Mutex::new(BufWriter::new(f)));

    // pre populate boards
    let mut fguard = f.lock().await;
    calc(
        999,
        boards[0].clone(),
        &mut boards,
        master_visited.clone(),
        &mut *master_item_counts.lock().await,
        &mut *fguard,
    )
    .await;
    drop(fguard);

    for b in boards.clone() {
        let mut fguard = f.lock().await;
        calc(
            999,
            b,
            &mut boards,
            master_visited.clone(),
            &mut *master_item_counts.lock().await,
            &mut *fguard,
        )
        .await;
    }

    for b in boards.clone() {
        b.show();
    }

    let mut handles = vec![];

    for thread in 0..boards.len() {
        let boards_clone = boards.clone();

        let master_visited_clone = master_visited.clone();
        let master_item_counts_clone = master_item_counts.clone();

        let visited_thread = Arc::new(Mutex::new(HashSet::new()));

        let handle = tokio::spawn(async move {
            let b = boards_clone[thread].clone();
            let mut thread_board = vec![b];
            let mut item_counts_clone = [0, 0, 0];

            let path = "boards_thread_".to_string()
                + &from_digit(thread as u32, 10).unwrap().to_string()
                + ".txt";
            let f = File::create(path).expect("unable to create file");
            let mut f = BufWriter::new(f);

            while let Some(b) = thread_board.pop() {
                calc(
                    thread,
                    b,
                    &mut thread_board,
                    visited_thread.clone(),
                    &mut item_counts_clone,
                    &mut f,
                )
                .await;

                if visited_thread.lock().await.len() > 1_000_000 {
                    merge_visited(
                        thread,
                        &mut *visited_thread.lock().await,
                        master_visited_clone.clone(),
                        &mut item_counts_clone,
                        master_item_counts_clone.clone(),
                    )
                    .await;
                }
            }

            // After the thread finishes its DFS, merge any remaining items in its visited HashSet into master_visited
            // let mut master_guard = master_visited_clone.lock().await;
            // master_guard.extend(visited_clone.iter().cloned());
            // drop(master_guard);
            //
            // visited_clone.clear();
            //
            // let mut master_item_guard = master_item_counts_clone.lock().await;
            // master_item_guard[0] += item_counts_clone[0];
            // master_item_guard[1] += item_counts_clone[1];
            // master_item_guard[2] += item_counts_clone[2];

            info!("Thread {} finished :)", thread);
        });

        handles.push(handle);
    }

    async fn merge_visited(
        thread_number: usize,
        visited_clone: &mut HashSet<Board>,
        master_visited_clone: Arc<Mutex<HashSet<Board>>>,
        item_counts_clone: &mut [u32; 3],
        master_item_counts_clone: Arc<Mutex<[u32; 3]>>,
    ) {
        info!("Thread {} is merging", thread_number);
        let mut master_guard = master_visited_clone.lock().await;
        master_guard.extend(visited_clone.iter().cloned());
        drop(master_guard);

        visited_clone.clear();

        let mut master_item_guard = master_item_counts_clone.lock().await;
        master_item_guard[0] += item_counts_clone[0];
        master_item_guard[1] += item_counts_clone[1];
        master_item_guard[2] += item_counts_clone[2];
        info!("Thread {} is done merging", thread_number);
    }

    let mut last_time = std::time::Instant::now();
    let stating_time = std::time::Instant::now();
    let mut old_total = 0;

    let stat_thread = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let stat_time = last_time.elapsed().as_millis();
            let master_item_counts_guard = master_item_counts.lock().await;
            let total = master_item_counts_guard.iter().sum::<u32>();

            info!("################################################################");
            info!(
                "enumerating... (WhiteWin: {}, BlackWin: {}, Intermediate: {}, total: {})",
                master_item_counts_guard[0],
                master_item_counts_guard[1],
                master_item_counts_guard[2],
                total
            );
            drop(master_item_counts_guard);

            info!(
                "Generation progress: {}%, in {}s",
                (total * 100) / 246803167,
                stating_time.elapsed().as_secs()
            );
            info!(
                "Current Generation Speed: {} Kboards/s",
                (total - old_total) as u128 / stat_time
            );
            info!(
                "Average Generation Speed: {} Kboards/s",
                total as u128 / stating_time.elapsed().as_millis()
            );
            info!(
                "Estimated time to finish: {}s",
                (246803167 - total) as u128 * stat_time / 1000 / 1000000
            );
            info!(
                "Estimated time to finish: {}m",
                (246803167 - total) as u128 * stat_time / 1000 / 1000000 / 60
            );
            info!("################################################################");
            last_time = std::time::Instant::now();
            old_total = total;
        }
    });

    stat_thread.await.unwrap();

    for handle in handles {
        handle.await.unwrap();
    }

    info!("All threads finished :)");
}

async fn calc(
    thread: usize,
    b: Board,
    boards: &mut Vec<Board>,
    visited: Arc<Mutex<HashSet<Board>>>,
    item_counts: &mut [u32; 3],
    f: &mut BufWriter<File>,
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

    writeln!(f, "{}", b.show_file()).unwrap();

    let mut vg = visited.lock().await;

    vg.insert(b);
    //item_counts[(1 - r) as usize] += 1;
    if vg.len() % 1_000_000 == 0 {
        info!("Thread {} still running: {} visited", thread, vg.len());
    }
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
