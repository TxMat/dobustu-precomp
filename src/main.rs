use std::error::Error;
use std::io::Write;

mod board_generator;
mod play_game;

fn main() {
    //play_game();
    board_generator::main();
}
