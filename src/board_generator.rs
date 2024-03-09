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

    let mut boards = vec![Board::init()];
    let mut visited = std::collections::HashSet::new();
    let mut item_counts = vec![0, 0, 0];

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

        write!(f, "{}\n", b.show_file()).unwrap();

        item_counts[(1 - r) as usize] += 1;
        if visited.len() % 1000000 == 0 {
            let total = item_counts[0] + item_counts[1] + item_counts[2];
            info!(
                "enumerating... (winning: {}, losing: {}, unknown: {}, total: {})",
                item_counts[0], item_counts[1], item_counts[2], total
            );
        }
    }
}
