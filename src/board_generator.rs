use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use log::info;

use game_helper::board::Board;
use game_helper::structs::GameResult;

pub(crate) fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let stating_time = std::time::Instant::now();
    let mut last_time = stating_time.clone();

    let mut boards = vec![Board::init()];
    let mut visited = std::collections::HashSet::new();
    let mut item_counts = [0, 0, 0];

    let path = "boards.txt";
    let f = File::create(path).expect("unable to create file");
    let mut f = BufWriter::new(f);

    while let Some(b) = boards.pop() {
        if visited.contains(&b) {
            continue;
        };
        visited.insert(b.clone());

        let r = match b.next() {
            GameResult::WhiteWin => 1,
            GameResult::BlackWin => 0,
            GameResult::Intermediate(bs) => {
                for b in bs {
                    boards.push(b)
                }
                -1
            }
        };

        writeln!(f, "{}", b.show_file()).unwrap();

        item_counts[(1 - r) as usize] += 1;
        if visited.len() % 2000000 == 0 {
            let stat_time = last_time.elapsed().as_millis();
            let total = item_counts[0] + item_counts[1] + item_counts[2];
            info!(
                "enumerating... (WhiteWin: {}, BlackWin: {}, Intermediate: {}, total: {})",
                item_counts[0], item_counts[1], item_counts[2], total
            );
            info!(
                "Generation progress: {}%, in {}s",
                (total * 100) / 246803167,
                stating_time.elapsed().as_secs()
            );
            info!(
                "Current Generation Speed: {} Mboards/s",
                2000000 / stat_time
            );
            info!(
                "Average Generation Speed: {} Mboards/s",
                total / stating_time.elapsed().as_millis()
            );
            info!(
                "Estimated time to finish: {}s",
                (246803167 - total) * stat_time / 1000 / 1000000
            );
            info!(
                "Estimated time to finish: {}m",
                (246803167 - total) * stat_time / 1000 / 1000000 / 60
            );
            info!("################################################################");
            last_time = std::time::Instant::now();
        }
    }
}
