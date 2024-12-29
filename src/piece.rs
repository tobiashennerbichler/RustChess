pub mod piece {
    use crate::board::board::Board;
    use crate::game::game::Game;
    use std::fmt;
    use std::ops::AddAssign;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Color {
        White,
        Black
    }
    
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Color::White => write!(f, "White"),
                Color::Black => write!(f, "Black")
            }
        }
    }
    
    impl Into<usize> for Color {
        fn into(self) -> usize {
            match self {
                Color::White => 0,
                Color::Black => 1
            }
        }
    }
    
    impl TryFrom<&str> for Color {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "w" => Ok(Color::White),
                "b" => Ok(Color::Black),
                _ => Err(())
            }
        }
    }
    
    impl Into<&str> for Color {
        fn into(self) -> &'static str {
            match self {
                Color::White => "w",
                Color::Black => "b"
            }
        }
    }
    
    #[derive(Copy, Clone, PartialEq)]
    pub struct Position {
        pub x: usize,
        pub y: usize
    }

    impl Position {
        fn get_move_to(&self, pos: Position) -> ChessMove {
            ChessMove {
                x: pos.x as i32 - self.x as i32,
                y: pos.y as i32 - self.y as i32
            }
        }
        
        fn add_move(&self, cmove: ChessMove) -> Result<Position, &'static str> {
            let x: i32 = self.x.try_into().unwrap();
            let y: i32 = self.y.try_into().unwrap();

            let (0..=7, 0..=7) = (x + cmove.x, y + cmove.y) else {
                return Err("Should not be possible to go out of bounds - check parser!");
            };
            Ok(Position {
                x: (x + cmove.x) as usize,
                y: (y + cmove.y) as usize
            })
        }
    }

    impl fmt::Debug for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let char_x: char = (self.x as u8 + b'a').try_into().expect("Must be valid ascii");
            let y = self.y + 1;
            write!(f, "{char_x}{y}")
        }
    }

    impl TryFrom<&str> for Position {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let trimmed = value.trim();
            if !trimmed.is_ascii() {
                return Err("Not a valid ascii string");
            }

            if trimmed.len() != 2 {
                return Err("String should contain two characters");
            }
            
            let bytes = value.as_bytes();
            match bytes {
                [b'a'..=b'h', b'1'..=b'8'] => {
                    let x = (bytes[0] - b'a') as usize;
                    let y = (bytes[1] - b'1') as usize;
                    Ok(Position {x, y})
                }
                _ => Err("Not a valid position string")
            }
        }
    }
    
    #[derive(Debug, Copy, Clone)]
    struct ChessMove {
        x: i32,
        y: i32
    }

    impl AddAssign for ChessMove {
        fn add_assign(&mut self, other: Self) {
            self.x += other.x;
            self.y += other.y;
        }
    }

    impl ChessMove {
        fn signum(&self) -> ChessMove {
            ChessMove {x: self.x.signum(), y: self.y.signum()}
        }
    }

    #[derive(Copy, Clone)]
    pub struct Piece {
        color: Color,
        position: Position,
        character: char,
        taken: bool,
        piece_type: PieceTypes
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum PieceTypes {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
    
    const CHAR_SET: [char; 12] = [
        '♟', '♞', '♝', '♜', '♛', '♚',
        '♙', '♘', '♗', '♖', '♕', '♔'
    ];

    impl Piece {
        pub fn new(piece_type: PieceTypes, color: Color, position: Position) -> Self {
            let taken = false;
            let offset = match color {
                Color::White => 0,
                Color::Black => 6
            };

            match piece_type {
                PieceTypes::Pawn => Piece {color, position, character: CHAR_SET[offset], taken, piece_type},
                PieceTypes::Knight => Piece {color, position, character: CHAR_SET[offset + 1], taken, piece_type},
                PieceTypes::Bishop => Piece {color, position, character: CHAR_SET[offset + 2], taken, piece_type},
                PieceTypes::Rook => Piece {color, position, character: CHAR_SET[offset + 3], taken, piece_type},
                PieceTypes::Queen => Piece {color, position, character: CHAR_SET[offset + 4], taken, piece_type},
                PieceTypes::King => Piece {color, position, character: CHAR_SET[offset + 5], taken, piece_type}
            }
        }

        pub fn is_taken(&self) -> bool {
            self.taken
        }

        pub fn take(&mut self) {
            self.taken = true;
            self.position = Position {x: 0, y: 0};
        }

        pub fn untake(&mut self, pos: Position) {
            self.taken = false;
            self.position = pos;
        }

        pub fn update_position(&mut self, new_pos: Position) {
            self.position = new_pos;
        }
        
        // TODO: handle en passant, castling and promotion
        // TODO: implement checkmate and stalemate
        pub fn is_field_reachable(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            match self.piece_type {
                PieceTypes::Pawn => self.is_field_reachable_pawn(new_pos, board),
                PieceTypes::Knight => self.is_field_reachable_knight(new_pos, board),
                PieceTypes::Bishop => self.is_field_reachable_bishop(new_pos, board),
                PieceTypes::Rook => self.is_field_reachable_rook(new_pos, board),
                PieceTypes::Queen => self.is_field_reachable_queen(new_pos, board),
                PieceTypes::King => self.is_field_reachable_king(new_pos, board)
            }
        }

        fn is_field_reachable_pawn(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let mut required_move = self.position.get_move_to(new_pos);
            let mut start_pos = 1;
            if let Color::Black = self.color {
                required_move.y *= -1;
                start_pos = 6;
            }
            
            match required_move {
                // Advance
                ChessMove {x: 0, y: 1..=2} => {
                    if required_move.y == 2 {
                        if self.position.y != start_pos {
                            return Err("Pawn cannot move twice if it has moved before");
                        }
                        
                        let direction = if self.color == Color::White { 1 } else { -1 };
                        if self.is_path_obstructed(board, required_move.y - 1, ChessMove {x: 0, y: direction}) {
                            return Err("Piece in the way");
                        }
                    }
                    
                    match board.get_board_entry(new_pos) {
                        Some(_) => Err("Pawn cannot move to populated field"),
                        None => Ok(())
                    }
                },
                // Take
                ChessMove {x, y: 1} if x.abs() == 1 => {
                    match board.get_board_entry(new_pos) {
                        Some(entry) if entry.player_color == self.color => Err("Cannot take own piece"),
                        Some(_) => Ok(()),
                        None => Err("Nothing to take")
                    }
                },
                _ => Err("Invalid Pawn move")
            }
        }
            
        fn is_field_reachable_knight(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x, y} if (x.abs() == 1 && y.abs() == 2) ||
                                             (x.abs() == 2 && y.abs() == 1) => {
                    self.check_takeable(self.color, new_pos, board)
                },
                _ => Err("Invalid Knight move")
            }
        }

        fn is_field_reachable_bishop(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x: 0, y: 0} => {
                    return Err("Invalid Bishop move");
                }
                ChessMove {x, y} if x == y || x == -y => {
                    if self.is_path_obstructed(board, x.abs() - 1, required_move.signum()) {
                        return Err("Piece in the way");
                    }
                 
                    self.check_takeable(self.color, new_pos, board)
                },
                _ => Err("Invalid Bishop move")
            }
        }

        fn is_field_reachable_rook(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x, y: 0} if x != 0 => {
                    if self.is_path_obstructed(board, x.abs() - 1, ChessMove {x: x.signum(), y: 0}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(self.color, new_pos, board)
                },
                ChessMove {x: 0, y} if y != 0 => {
                    if self.is_path_obstructed(board, y.abs() - 1, ChessMove {x: 0, y: y.signum()}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(self.color, new_pos, board)
                },
                _ => Err("Invalid Rook move")
            }
        }

        fn is_field_reachable_queen(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let reachable_rook = self.is_field_reachable_rook(new_pos, board);
            let reachable_bishop = self.is_field_reachable_bishop(new_pos, board);
            
            match (reachable_rook, reachable_bishop) {
                (Err(_), Err(_)) => Err("Invalid Queen move"),
                _ => Ok(())
            }
        }

        fn is_field_reachable_king(&self, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x: 0, y: 0} => Err("Invalid King move"),
                ChessMove {x, y} if x.abs() <= 1 && y.abs() <= 1 => {
                    self.check_takeable(self.color, new_pos, board)
                },
                _ => Err("Invalid King move")
            }
        }
        
        fn is_path_obstructed(&self, board: &Board, num_fields: i32, add: ChessMove) -> bool {
            let mut temp_pos = self.position;
            for _ in 0..num_fields {
                temp_pos = temp_pos.add_move(add).expect("Should not go out of bounds");

                if let Some(_) = board.get_board_entry(temp_pos) {
                    return true;
                }
            }

            false
        }

        fn check_takeable(&self, piece_color: Color, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            match board.get_board_entry(new_pos) {
                Some(entry) if entry.player_color == piece_color => Err("Cannot take own piece"),
                Some(_) => Ok(()),
                None => Ok(())
            }
        }

        pub fn get_legal_positions(&self, game: &mut Game) -> Vec<Position> {
            let mut positions = self.get_reachable_positions(game.get_ref_board());
            
            let mut indx = 0;
            let mut len = positions.len();
            while indx < len {
                match game.does_move_cause_check(self.get_position(), positions[indx]) {
                    Ok(_) => indx += 1,
                    Err(_) => {
                        positions.swap_remove(indx);
                        len -= 1;
                    }
                }
            }
            
            positions
        }

        pub fn get_reachable_positions(&self, board: &Board) -> Vec<Position> {
            match self.piece_type {
                PieceTypes::Pawn => self.get_reachable_positions_pawn(board),
                PieceTypes::Knight => self.get_reachable_positions_knight(board),
                PieceTypes::Bishop => self.get_reachable_positions_bishop(board),
                PieceTypes::Rook => self.get_reachable_positions_rook(board),
                PieceTypes::Queen => self.get_reachable_positions_queen(board),
                PieceTypes::King => self.get_reachable_positions_king(board)
            }
        }
        
        fn get_reachable_positions_pawn(&self, board: &Board) -> Vec<Position> {
            let mut positions = Vec::new();
            let sign = match self.color {
                Color::White => 1,
                Color::Black => -1
            };

            for step in 1..=2 {
                let cmove = ChessMove {x: 0, y: sign * step};
                let Ok(new_pos) = self.position.add_move(cmove) else {
                    continue;
                };
                if let Ok(_) = self.is_field_reachable_pawn(new_pos, board) {
                    positions.push(new_pos);
                }
            }
            
            for add in [-1, 1] {
                let cmove = ChessMove {x: add, y: sign};
                let Ok(new_pos) = self.position.add_move(cmove) else {
                    continue;
                };
                if let Ok(_) = self.is_field_reachable_pawn(new_pos, board) {
                    positions.push(new_pos);
                }
            }

            positions
        }
        
        fn get_reachable_positions_knight(&self, board: &Board) -> Vec<Position> {
            let mut positions = Vec::new();
            
            for move_x in [-1, 1] {
                for move_y in [-2, 2] {
                    let cmove = ChessMove {x: move_x, y: move_y};
                    let transposed_move = ChessMove {x: move_y, y: move_x};

                    if let Ok(new_pos) = self.position.add_move(cmove) {
                        if let Ok(_) = self.is_field_reachable_knight(new_pos, board) {
                            positions.push(new_pos);
                        }
                    }

                    if let Ok(transposed_pos) = self.position.add_move(transposed_move) {
                        if let Ok(_) = self.is_field_reachable_knight(transposed_pos, board) {
                            positions.push(transposed_pos);
                        }
                    } 
                }
            }

            positions
        }
        
        fn get_reachable_positions_bishop(&self, board: &Board) -> Vec<Position> {
            let mut positions= Vec::new();
            
            for move_x in [-1, 1] {
                for move_y in [-1, 1] {
                    let mut cmove = ChessMove {x: 0, y: 0};
                    for _ in 0..7 {
                        cmove += ChessMove {x: move_x, y: move_y};
                        let Ok(new_pos) = self.position.add_move(cmove) else {
                            break;
                        };
                        if let Ok(_) = self.is_field_reachable_bishop(new_pos, board) {
                            positions.push(new_pos);
                        }
                    }
                }
            }

            positions
        }

        fn get_reachable_positions_rook(&self, board: &Board) -> Vec<Position> {
            let mut positions= Vec::new();

            for x in 0..=7 {
                let curr_pos = Position {x, y: self.position.y};
                if let Err(_) = self.is_field_reachable_rook(curr_pos, board) {
                    continue;
                }

                positions.push(curr_pos);
            }
            
            for y in 0..=7 {
                let curr_pos = Position {x: self.position.x, y};
                if let Err(_) = self.is_field_reachable_rook(curr_pos, board) {
                    continue;
                }

                positions.push(curr_pos);
            }
            
            positions 
        }
        
        fn get_reachable_positions_queen(&self, board: &Board) -> Vec<Position> {
            let mut positions= Vec::new();
            
            positions.append(&mut self.get_reachable_positions_rook(board));
            positions.append(&mut self.get_reachable_positions_bishop(board));
            
            positions
        }

        fn get_reachable_positions_king(&self, board: &Board) -> Vec<Position> {
            let mut positions = Vec::new();
            
            for move_x in [-1, 0, 1] {
                for move_y in [-1, 0, 1] {
                    let cmove = ChessMove {x: move_x, y: move_y};
                    let Ok(new_pos) = self.position.add_move(cmove) else {
                        continue;
                    };
                    
                    if let Err(_) = self.is_field_reachable_king(new_pos, board) {
                        continue;
                    }
                    
                    positions.push(new_pos);
                }
            }

            positions
        }
        
        pub fn get_position(&self) -> Position {
            self.position
        }

        pub fn get_piece_type(&self) -> PieceTypes {
            self.piece_type
        }
    }
    
    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.character)
        }
    }
    
    impl fmt::Debug for Piece {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.taken {
                true => Ok(()),
                false => write!(f, "{} @ {:?}", self.character, self.position)
            }
        }
    }

}