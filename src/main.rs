mod board_generator;
mod board_generator_2;
mod board_generator_multi_threaded;
mod play_game;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    //play_game();
    //board_generator::main();
    board_generator_multi_threaded::main().await;
}
