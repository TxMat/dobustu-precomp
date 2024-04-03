use log::set_max_level;
use board::Board;
use piece::Piece;
use structs::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NextMove(pub u16);

impl NextMove {
    pub fn new(piece: Piece, old_pos: Position, new_pos: Position) -> Self {
        let piece_pos_pos: u16 = ((piece.0 as u16) << 8)
            + ((<&Position as Into<u8>>::into(&old_pos) << 4)
            + <&Position as Into<u8>>::into(&new_pos)) as u16;
        // println!("new piece pos: {:X}", piece_pos_pos);
        NextMove(piece_pos_pos)
    }

    pub fn get_current_and_next(&self, is_player_1: bool) -> (u8, u8)
    {
        let mut newPosition:Position = Position::from(self.0 & 0x00f);
        let mut oldPosition:Position = Position::from((self.0 & 0x0f0) >> 4);
        let mut piece:Piece = Piece::from()
    }

    pub fn new_from_board(initial_board: &Board, next_board: &Board) -> Self {
        let initial_byte_arr = initial_board.0.to_be_bytes();
        let next_byte_arr = next_board.0.to_be_bytes();

        // find the difference between the two boards
        for i in 0..7 {
            if initial_byte_arr[i] != next_byte_arr[i] {
                let piece = (next_byte_arr[i] & 0xf0) >> 4;
                let new_pos = next_byte_arr[i] & 0x0f;
                if new_pos == Position::Dead as u8 {
                    continue;
                }
                let old_pos = initial_byte_arr[i] & 0x0f;
                // opti ?
                return NextMove(((piece as u16) << 8) + ((old_pos << 4) + new_pos) as u16);
            }
        }
        return NextMove(0);
    }
}