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
    
    #[derive(Debug, Copy, Clone)]
    pub struct Position {
        pub x: usize,
        pub y: usize
    }

    impl Position {
        fn get_distance_to(&self, pos: Position) -> Distance {
            Distance {
                x: pos.x as i32 - self.x as i32,
                y: pos.y as i32 - self.y as i32
            }
        }
        
        fn add_distance(&mut self, distance: Distance) {
            let x: i32 = self.x.try_into().unwrap();
            let y: i32 = self.y.try_into().unwrap();

            let (0..=7, 0..=7) = (x + distance.x, y + distance.y) else {
                panic!("Should not be possible to go out of bounds - check parser!");
            };
            self.x = (self.x as i32 + distance.x) as usize;
            self.y = (self.y as i32 + distance.y) as usize;
        }
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Distance {
        x: i32,
        y: i32
    }

    impl AddAssign for Distance {
        fn add_assign(&mut self, other: Self) {
            self.x += other.x;
            self.y += other.y;
        }
    }

    impl Distance {
        fn signum(&self) -> Distance {
            Distance {x: self.x.signum(), y: self.y.signum()}
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

        pub fn update_position(&mut self, new_pos: Position) {
            self.position = new_pos;
        }
        
        // TODO: check if player in check, otherwise restrict to king moves
        // TODO: check for possible exposed checks after move --> test move and run update_check
        // method
        // TODO: maybe is_legal_move does not check for exposed checks, instead do it afterwards
        // and use is_legal_move functions to check all pieces for new_pos = king.pos
        // TODO: handle en passant, castling and promotion
        // TODO: implement checkmate and stalemate
        pub fn is_legal_move(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            match self.piece_type {
                PieceTypes::Pawn => self.is_legal_move_pawn(player, new_pos, board),
                PieceTypes::Knight => self.is_legal_move_knight(player, new_pos, board),
                PieceTypes::Bishop => self.is_legal_move_bishop(player, new_pos, board),
                PieceTypes::Rook => self.is_legal_move_rook(player, new_pos, board),
                PieceTypes::Queen => self.is_legal_move_queen(player, new_pos, board),
                PieceTypes::King => self.is_legal_move_king(player, new_pos, board)
            }
        }

        fn is_legal_move_pawn(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let mut distance = self.position.get_distance_to(new_pos);
            let mut start_pos = 1;
            if let Color::Black = self.color {
                distance.y *= -1;
                start_pos = 6;
            }
            
            match distance {
                // Advance
                Distance {x: 0, y: 1..=2} => {
                    if distance.y == 2 && self.position.y != start_pos {
                        return Err("Pawn cannot move twice if it has moved before");
                    }
                    
                    match board.get_board_entry(new_pos) {
                        Some(_) => Err("Pawn cannot move to populated field"),
                        None => Ok(())
                    }
                },
                // Take
                Distance {x, y: 1} if x.abs() == 1 => {
                    match board.get_board_entry(new_pos) {
                        Some(entry) if entry.player_color == player.get_color() => Err("Cannot take own piece"),
                        Some(_) => Ok(()),
                        None => Err("Nothing to take")
                    }
                },
                _ => Err("Invalid Pawn move")
            }
        }
            
        fn is_legal_move_knight(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let distance = self.position.get_distance_to(new_pos);

            match distance {
                Distance {x, y} if (x.abs() == 1 && y.abs() == 2) ||
                                             (x.abs() == 2 && y.abs() == 1) => {
                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Knight move")
            }
        }

        fn is_legal_move_bishop(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let distance = self.position.get_distance_to(new_pos);

            match distance {
                Distance {x: 0, y: 0} => {
                    return Err("Invalid Bishop move");
                }
                Distance {x, y} if x == y || x == -y => {
                    if self.is_path_obstructed(board, x.abs() - 1, distance.signum()) {
                        return Err("Piece in the way");
                    }
                 
                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Bishop move")
            }
        }

        fn is_legal_move_rook(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let distance = self.position.get_distance_to(new_pos);

            match distance {
                Distance {x, y: 0} if x != 0 => {
                    if self.is_path_obstructed(board, x.abs() - 1, Distance {x: x.signum(), y: 0}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                Distance {x: 0, y} if y != 0 => {
                    if self.is_path_obstructed(board, y.abs() - 1, Distance {x: 0, y: y.signum()}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Rook move")
            }
        }

        fn is_legal_move_queen(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let distance = self.position.get_distance_to(new_pos);

            match distance {
                Distance {x: 0, y: 0} => Err("Invalid Queen move"),
                Distance {x, y: 0} => {
                    if self.is_path_obstructed(board, x.abs() - 1, Distance {x: x.signum(), y: 0}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                Distance {x: 0, y} => {
                    if self.is_path_obstructed(board, y.abs() - 1, Distance {x: 0, y: y.signum()}) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                Distance {x, y} if x == y || x == -y => {
                    if self.is_path_obstructed(board, x.abs() - 1, distance.signum()) {
                        return Err("Piece in the way");
                    }

                    self.check_takeable(board, player.get_color(), new_pos)
                },
                _ => Err("Invalid Queen move")
            }
        }

        fn is_legal_move_king(&self, player: &Player, new_pos: Position, board: &Board) -> Result<(), &'static str> {
            let distance = self.position.get_distance_to(new_pos);

            match distance {
                Distance {x: 0, y: 0} => Err("Invalid King move"),
                Distance {x, y} if x <= 1 && y <= 1 => {
                    self.check_takeable(board, player.get_color(), new_pos)
                }
                _ => Err("Invalid King move")
            }
        }

        fn is_path_obstructed(&self, board: &Board, num_fields: i32, add: Distance) -> bool {
            let mut temp_pos = self.position;
            for _ in 0..num_fields {
                temp_pos.add_distance(add);

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
                true => write!(f, "{} already taken", self.character),
                false => write!(f, "{} @ {}/{}", self.character, self.position.x, self.position.y)
            }
        }
    }

}