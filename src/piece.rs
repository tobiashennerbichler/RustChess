pub mod piece {
    use crate::board::board::Board;
    use crate::player::player::Player;
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
    
    #[derive(Copy, Clone)]
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
        
        fn add_move(&mut self, cmove: ChessMove) {
            let x: i32 = self.x.try_into().unwrap();
            let y: i32 = self.y.try_into().unwrap();

            let (0..=7, 0..=7) = (x + cmove.x, y + cmove.y) else {
                panic!("Should not be possible to go out of bounds - check parser!");
            };
            self.x = (self.x as i32 + cmove.x) as usize;
            self.y = (self.y as i32 + cmove.y) as usize;
        }
    }

    impl TryFrom<(i32, i32)> for Position {
        type Error = &'static str;

        fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
            if value.0 < 0 || value.0 > 7 {
                Err("x out of bounds")
            } else if value.1 < 0 || value.1 > 7 {
                Err("y out of bounds")
            } else {
                Ok(Position { x: value.0 as usize, y: value.1 as usize})
            }
        }
    }
    
    impl fmt::Debug for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let char_x: char = (self.x as u8 + b'a').try_into().expect("Must be valid ascii");
            let y = self.y + 1;
            write!(f, "{char_x}{y}")
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
        pub fn is_field_reachable(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            if new_pos.x > 7 || new_pos.y > 7 {
                panic!("Out-of-bounds access");
            }

            match self.piece_type {
                PieceTypes::Pawn => self.is_field_reachable_pawn(player, new_pos, board),
                PieceTypes::Knight => self.is_field_reachable_knight(player, new_pos, board),
                PieceTypes::Bishop => self.is_field_reachable_bishop(player, new_pos, board),
                PieceTypes::Rook => self.is_field_reachable_rook(player, new_pos, board),
                PieceTypes::Queen => self.is_field_reachable_queen(player, new_pos, board),
                PieceTypes::King => self.is_field_reachable_king(player, new_pos, board)
            }
        }

        fn is_field_reachable_pawn(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let mut required_move = self.position.get_move_to(new_pos);
            let mut start_pos = 1;
            if let Color::Black = self.color {
                required_move.y *= -1;
                start_pos = 6;
            }
            
            match required_move {
                // Advance
                ChessMove {x: 0, y: 1..=2} => {
                    if required_move.y == 2 && self.position.y != start_pos {
                        return Err("Pawn cannot move twice if it has moved before");
                    }
                    
                    match board.get_board_entry(new_pos) {
                        Some(_) => Err("Pawn cannot move to populated field"),
                        None => Ok(())
                    }
                },
                // Take
                ChessMove {x, y: 1} if x.abs() == 1 => {
                    match board.get_board_entry(new_pos) {
                        Some(entry) if entry.player_color == player.get_color() => Err("Cannot take own piece"),
                        Some(_) => Ok(()),
                        None => Err("Nothing to take")
                    }
                },
                _ => Err("Invalid Pawn move")
            }
        }
            
        fn is_field_reachable_knight(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x, y} if (x.abs() == 1 && y.abs() == 2) ||
                                             (x.abs() == 2 && y.abs() == 1) => {
                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Knight move")
            }
        }

        fn is_field_reachable_bishop(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x: 0, y: 0} => {
                    return Err("Invalid Bishop move");
                }
                ChessMove {x, y} if x == y || x == -y => {
                    if self.is_path_obstructed(board, x.abs() - 1, required_move.signum()) {
                        return Err("Piece in the way");
                    }
                 
                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Bishop move")
            }
        }

        fn is_field_reachable_rook(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x, y: 0} if x != 0 => {
                    if self.is_path_obstructed(board, x.abs() - 1, ChessMove {x: x.signum(), y: 0}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                ChessMove {x: 0, y} if y != 0 => {
                    if self.is_path_obstructed(board, y.abs() - 1, ChessMove {x: 0, y: y.signum()}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Rook move")
            }
        }

        fn is_field_reachable_queen(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let reachable_rook = self.is_field_reachable_rook(player, new_pos, board);
            let reachable_bishop = self.is_field_reachable_bishop(player, new_pos, board);
            
            match (reachable_rook, reachable_bishop) {
                (Err(_), Err(_)) => Err("Invalid Queen move"),
                _ => Ok(())
            }
        }

        fn is_field_reachable_king(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let required_move = self.position.get_move_to(new_pos);

            match required_move {
                ChessMove {x: 0, y: 0} => Err("Invalid King move"),
                ChessMove {x, y} if x.abs() <= 1 && y.abs() <= 1 => {
                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid King move")
            }
        }
        
        fn is_path_obstructed(&self, board: &Board, num_fields: i32, add: ChessMove) -> bool {
            let mut temp_pos = self.position;
            for _ in 0..num_fields {
                temp_pos.add_move(add);

                if let Some(_) = board.get_board_entry(temp_pos) {
                    return true;
                }
            }

            false
        }

        fn check_takeable(&self, board: &Board, player_color: Color, new_pos: Position) -> Result<(), &'static str> {
            match board.get_board_entry(new_pos) {
                Some(entry) if entry.player_color == player_color => Err("Cannot take own piece"),
                Some(_) => Ok(()),
                None => Ok(())
            }
        }

        pub fn get_reachable_positions(&self, player: &Player, board: &Board) -> Vec<Position> {
            match self.piece_type {
                PieceTypes::Pawn => self.get_reachable_positions_pawn(player, board),
                PieceTypes::Knight => Vec::new(),
                PieceTypes::Bishop => self.get_reachable_positions_bishop(player, board),
                PieceTypes::Rook => self.get_reachable_positions_rook(player, board),
                PieceTypes::Queen => self.get_reachable_positions_queen(player, board),
                PieceTypes::King => Vec::new()
            }
        }
        
        fn get_reachable_positions_pawn(&self, player: &Player, board: &Board) -> Vec<Position> {
            let mut moves = Vec::new();

            let x = self.position.x as i32;
            for step in 1..=2 {
                let add = match self.color {
                    Color::White => step,
                    Color::Black => -step
                };
                
                let y = self.position.y as i32 + add;
                let Ok(new_pos) = Position::try_from((x, y)) else {
                    continue;
                };
                if let Ok(_) = self.is_field_reachable_pawn(player, new_pos, board) {
                    moves.push(new_pos);
                }
            }
            
            let y = self.position.y as i32 + 1;
            for add in [-1, 1] {
                let x = self.position.x as i32 + add;
                let Ok(new_pos) = Position::try_from((x, y)) else {
                    continue;
                };
                if let Ok(_) = self.is_field_reachable_pawn(player, new_pos, board) {
                    moves.push(new_pos);
                }
            }

            moves
        }
        
        fn get_reachable_positions_bishop(&self, player: &Player, board: &Board) -> Vec<Position> {
            let mut moves = Vec::new();
            
            for add_x in [-1, 1] {
                for add_y in [-1, 1] {
                    let mut x = self.position.x as i32;
                    let mut y = self.position.y as i32;
                    for _ in 0..7 {
                        x += add_x;
                        y += add_y;
                        
                        let Ok(new_pos) = Position::try_from((x, y)) else {
                            break;
                        };
                        if let Ok(_) = self.is_field_reachable_bishop(player, new_pos, board) {
                            moves.push(new_pos);
                        }
                    }
                }
            }

            moves
        }

        fn get_reachable_positions_rook(&self, player: &Player, board: &Board) -> Vec<Position> {
            let mut moves = Vec::new();

            let y = self.position.y;
            for x in 0..=7 {
                let curr_pos = Position {x, y};
                if let Err(_) = self.is_field_reachable_rook(player, curr_pos, board) {
                    continue;
                }

                moves.push(curr_pos);
            }
            
            let x = self.position.x;
            for y in 0..=7 {
                let curr_pos = Position {x, y};
                if let Err(_) = self.is_field_reachable_rook(player, curr_pos, board) {
                    continue;
                }

                moves.push(curr_pos);
            }
            
            moves
        }
        
        fn get_reachable_positions_queen(&self, player: &Player, board: &Board) -> Vec<Position> {
            let mut moves = Vec::new();
            
            moves.append(&mut self.get_reachable_positions_rook(player, board));
            moves.append(&mut self.get_reachable_positions_bishop(player, board));
            
            moves
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
                false => write!(f, "{} @ {}/{}", self.character, self.position.x, self.position.y)
            }
        }
    }

}