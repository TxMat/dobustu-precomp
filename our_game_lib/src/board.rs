/// todo divide the code in smaller functions
use std::error::Error;
use std::hash::Hash;
// Importing the necessary modules and structs for the Board struct
use moves::Move;
use piece::Color::{Black, White};
use piece::Piece;
use piece::PieceType::{Chick, Elephant, Giraffe, Lion};
use piece::{Color, PieceType};
use structs::GameError;
use structs::GameResult::{BlackWin, Intermediate, WhiteWin};

// The `derive` attribute automatically implements the specified traits for the struct.
// Clone: Allows the struct to be duplicated.
// PartialEq and Eq: Allows the struct to be compared for equality.
// Hash: Allows the struct to be used as a key in a HashMap.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board {
    // 2D array of 3x4 pieces. `Option` is a Rust enum that can either be `Some` or `None`.
    // `Some` indicates that there is a value and `None` indicates that there is no value.
    board: [[Option<Piece>; 3]; 4],
    // Cemetery for the white pieces. `Vec` is a growable, heap-allocated data structure.
    white_cemetery: Vec<Piece>,
    // Cemetery for the black pieces.
    black_cemetery: Vec<Piece>,
    // Boolean to keep track of whose turn it is.
    is_white_turn: bool,
    // The winner of the game, if there is one.
    pub winner: Option<Color>,
}

impl Board {
    // Creates a new empty board.
    pub fn new_empty() -> Board {
        Board {
            board: [[None; 3]; 4],
            white_cemetery: Vec::new(),
            black_cemetery: Vec::new(),
            is_white_turn: true,
            winner: None,
        }
    }

    // Returns the piece at the given coordinates.
    // The coordinates are flipped because of the way the board is stored.
    pub fn get(&self, x: usize, y: usize) -> Option<Piece> {
        self.board[y][x]
    }

    // Places a piece at the given coordinates.
    pub fn put(&mut self, x: usize, y: usize, p: &Piece) {
        self.board[y][x] = Some(*p);
    }

    // Removes the piece at the given coordinates.
    pub fn del(&mut self, x: usize, y: usize) {
        self.board[y][x] = None;
    }

    // Initializes the board with the default setup.
    pub fn init() -> Board {
        let mut b = Board::new_empty();
        b.put(0, 0, &Piece::new(Elephant, White, false));
        b.put(1, 0, &Piece::new(Lion, White, false));
        b.put(2, 0, &Piece::new(Giraffe, White, false));
        b.put(1, 1, &Piece::new(Chick, White, false));
        b.put(1, 2, &Piece::new(Chick, Black, false));
        b.put(2, 3, &Piece::new(Elephant, Black, false));
        b.put(1, 3, &Piece::new(Lion, Black, false));
        b.put(0, 3, &Piece::new(Giraffe, Black, false));
        b
    }

    // Finds a piece on the board and returns its coordinates.
    pub fn find_piece_on_board(&self, piece: &Piece) -> Result<(usize, usize), Box<dyn Error>> {
        for y in 0..4 {
            for x in 0..3 {
                if self.get(x, y) == Some(*piece) {
                    return Ok((x, y));
                }
            }
        }
        Err(GameError::PieceNotInBoard.into())
    }

    pub fn get_curent_player_piece_list(&self) -> Vec<Piece> {
        let mut result = Vec::new();
        for y in 0..4 {
            for x in 0..3 {
                if let Some(p) = self.get(x, y) {
                    if p.color == self.get_turn() {
                        result.push(p);
                    }
                }
            }
        }
        result
    }

    pub fn get_legal_move_list_for_piece<'a>(
        &'a self,
        piece: &'a Piece,
    ) -> Result<Vec<&Move>, Box<dyn Error>> {
        let mut result = Vec::new();
        let piece_pos = self.find_piece_on_board(piece)?;

        let moves: Vec<&Move> = piece
            .piece_type
            .moves()
            .iter()
            .map(|m| if piece.color == Black { m.invert() } else { *m })
            .collect();

        for m in moves {
            let (x, y) = (piece_pos.0 as i8 + m.x, piece_pos.1 as i8 + m.y);
            if x > 2 || y > 3 || x < 0 || y < 0 {
                continue;
            }
            let target_pos = (x as usize, y as usize);
            let piece_on_the_way = self.get(target_pos.0, target_pos.1);
            if let Some(p) = piece_on_the_way {
                if p.color != piece.color {
                    result.push(m);
                }
            } else {
                result.push(m);
            }
        }
        Ok(result)
    }

    pub fn get_curent_player_cemetery(&self) -> Vec<Piece> {
        match self.get_turn() {
            White => self.white_cemetery.clone(),
            Black => self.black_cemetery.clone(),
        }
    }

    // Moves a piece on the board if the move is valid.
    // Returns a `Result` type which is an enum that can either be `Ok` or `Err`.
    // `Ok` indicates a successful operation and `Err` indicates an error.
    pub fn move_piece(&mut self, piece: &mut Piece, move_: &Move) -> Result<(), Box<dyn Error>> {
        // Check if the move is valid for the piece.
        if self.get_legal_move_list_for_piece(piece)?.contains(&&move_) {
            // Set the color of the piece to the current turn's color.
            piece.color = self.get_turn();
            // Find the piece on the board.
            let piece_pos = self.find_piece_on_board(piece)?;

            // Calculate the new coordinates for the piece.
            let (x, y) = (piece_pos.0 as i8 + move_.x, piece_pos.1 as i8 + move_.y);
            // Check if the new coordinates are out of bounds.
            if x > 2 || y > 3 || x < 0 || y < 0 {
                return Err(GameError::OutOfBounds.into());
            }

            // Get the color of the current player.
            let current_player_color = self.get_turn();

            // Get the piece at the target position.
            let target_pos = (x as usize, y as usize);
            let piece_on_the_way = self.get(target_pos.0, target_pos.1);

            // If there is a piece at the target position, check if it's the same color as the current player.
            if let Some(mut p) = piece_on_the_way {
                if p.color == current_player_color {
                    return Err(GameError::IllegalMove.into());
                }
                // If the piece at the target position is a Lion, the current player wins.
                match p.piece_type {
                    Lion => self.winner = Some(current_player_color),
                    PieceType::Hen => p.piece_type = Chick,
                    _ => (),
                }
                if p.is_duplicated {
                    p.is_duplicated = false;
                }
                // Add the piece at the target position to the current player's cemetery.
                let cemetery = match current_player_color {
                    White => &mut self.white_cemetery,
                    Black => &mut self.black_cemetery,
                };
                p.color = current_player_color;
                cemetery.push(p);
            }

            // Remove the piece from its current position.
            self.del(piece_pos.0, piece_pos.1);

            // If the piece is a Chick and it reaches the last row, it becomes a Hen.
            if piece.piece_type == Chick && (target_pos.1 == 0 || target_pos.1 == 3) {
                let mut hen = Piece::new(PieceType::Hen, current_player_color, false);
                if self.find_piece_on_board(&hen).is_ok() {
                    hen.is_duplicated = true;
                }
                self.put(target_pos.0, target_pos.1, &hen);
            } else {
                // Otherwise, move the piece to the target position.
                self.put(target_pos.0, target_pos.1, piece);
            }

            // If the piece is a Lion and it reaches the last row, the game is over.
            if piece.piece_type == Lion
                && ((current_player_color == White && target_pos.1 == 3)
                    || (current_player_color == Black && target_pos.1 == 0))
            {
                self.winner = Some(current_player_color);
            }

            // Switch turns.
            self.is_white_turn = !self.is_white_turn;
            Ok(())
        } else {
            // If the move is not valid, return an error.
            Err(GameError::IllegalMove.into())
        }
    }

    // Returns the color of the current turn.
    pub fn get_turn(&self) -> Color {
        if self.is_white_turn {
            White
        } else {
            Black
        }
    }

    // Drops a piece onto the board.
    pub fn drop_piece(
        &mut self,
        piece: &mut Piece,
        x: usize,
        y: usize,
    ) -> Result<(), Box<dyn Error>> {
        // Check if the target position is occupied.
        if self.get(x, y).is_some() {
            Err(GameError::IllegalMove.into())
        } else {
            // Check if the piece is in the current player's cemetery.
            let cemetary = match piece.color {
                White => &mut self.white_cemetery,
                Black => &mut self.black_cemetery,
            };
            if cemetary.contains(piece) {
                // If the piece is in the cemetery, remove it from the cemetery and place it on the board.
                cemetary.retain(|p| p != piece);
                if self.find_piece_on_board(piece).is_ok() {
                    piece.is_duplicated = true;
                }
                self.put(x, y, piece);
                // Switch turns.
                self.is_white_turn = !self.is_white_turn;
                Ok(())
            } else {
                // If the piece is not in the cemetery, return an error.
                // better error
                Err(GameError::IllegalMove.into())
            }
        }
    }

    // Checks if the game is over.
    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }

    // Prints the current state of the board and the cemeteries.
    pub fn show(&self) {
        println!("##############");

        fn print_row_nb() {
            print!("    0 1 2")
        }

        for y in (0..6).rev() {
            match y {
                0 => print_row_nb(),
                5 => print_row_nb(),
                _ => self.a(y),
            }

            println!();
        }
        println!("##############\n");
        println!(
            "White cemetery: {:?}",
            self.white_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
        println!(
            "Black cemetery: {:?}",
            self.black_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        );
    }

    fn a(&self, y: usize) {
        for x in 0..5 {
            let y = y - 1;
            match x {
                0 => print!("{} | ", y),
                4 => print!("| {}", y),
                _ => print!(
                    "{} ",
                    match self.get(x - 1, y) {
                        Some(p) => p.show(),
                        None => " ".to_string(),
                    }
                ),
            }
        }
    }

    fn a2(&self, y: usize, s: &mut String) {
        for x in 0..5 {
            let y = y - 1;
            match x {
                0 => s.push_str(&format!("{} | ", y)),
                4 => s.push_str(&format!("| {}", y)),
                _ => s.push_str(&format!(
                    "{} ",
                    match self.get(x - 1, y) {
                        Some(p) => p.show(),
                        None => " ".to_string(),
                    }
                )),
            }
        }
    }

    pub fn get_hash(&self) -> String {
        let mut s = String::new();
        for y in 0..4 {
            for x in 0..3 {
                match self.get(x, y) {
                    Some(p) => {
                        s.push_str(&format!("{}{}{}", p.show(), x, y));
                    }
                    None => s.push_str(&format!("#{}{}", x, y)),
                }
            }
        }
        s
    }

    pub fn show_file(&self) -> String {
        let mut s = String::new();
        s.push_str(self.get_hash().as_str());
        s.push_str("\n");
        s.push_str("##############\n");

        for y in (0..6).rev() {
            match y {
                0 => s.push_str("    0 1 2"),
                5 => s.push_str("    0 1 2"),
                _ => self.a2(y, &mut s),
            }

            s.push_str("\n");
        }
        s.push_str("##############\n");
        s.push_str(&format!(
            "White cemetery: {:?}",
            self.white_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        ));
        s.push_str("\n");
        s.push_str(&format!(
            "Black cemetery: {:?}",
            self.black_cemetery
                .iter()
                .map(|p| p.show().to_string())
                .collect::<Vec<String>>()
        ));
        s.push_str("\n\n");
        s
    }

    pub fn next(&self) -> crate::structs::GameResult {
        let mut boards = vec![];
        for y in 0..4 {
            for x in 0..3 {
                let p = self.get(x, y);
                match p {
                    Some(p) => {
                        if p.color != self.get_turn() {
                            continue;
                        }
                        for m in self.get_legal_move_list_for_piece(&p).unwrap() {
                            let mut b = self.clone();
                            b.move_piece(&mut p.clone(), m).unwrap();
                            if b.is_game_over() {
                                match b.winner {
                                    Some(Color::White) => return WhiteWin,
                                    Some(Color::Black) => return BlackWin,
                                    _ => (),
                                }
                            }
                            boards.push(b.clone());
                        }
                    }
                    None => {
                        for p in self.get_curent_player_cemetery() {
                            for y in 0..4 {
                                for x in 0..3 {
                                    let mut b = self.clone();
                                    if b.drop_piece(&mut p.clone(), x, y).is_ok() {
                                        boards.push(b);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Intermediate(boards)
    }
}
