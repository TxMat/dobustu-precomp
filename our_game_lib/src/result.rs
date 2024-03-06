use board::Board;

#[derive(PartialEq, Eq)]
pub enum Result {
    Win,
    Lose,
    Intermediate(Vec<Board>)
}