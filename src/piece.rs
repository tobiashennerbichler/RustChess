pub mod piece {
    use crate::board::board::Board;
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
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
    
    #[derive(Debug, Copy, Clone)]
    pub struct Position {
        pub x: usize,
        pub y: usize
    }

    impl Position {
        fn get_distance_between(&self, pos: &Position) -> (i32, i32) {
            (pos.x as i32 - self.x as i32, pos.y as i32 - self.y as i32)
        }
    }
    
    pub struct Piece {
        color: Color,
        position: Position,
        character: &'static str,
        taken: bool,
        piece_type: PieceTypes
    }
    
    impl Piece {
        pub fn get_position(&self) -> Position {
            self.position
        }
        
        pub fn get_piece_type(&self) -> PieceTypes {
            self.piece_type
        }
        
        pub fn get_color(&self) -> Color {
            self.color
        }
        
        pub fn is_legal_move(&self, new_pos: &Position, board: &Board) -> bool {
            match self.piece_type {
                PieceTypes::Pawn => self.is_legal_move_pawn(new_pos, board),
                PieceTypes::Knight => self.is_legal_move_knight(new_pos, board),
                PieceTypes::Bishop => self.is_legal_move_bishop(new_pos, board),
                PieceTypes::Rook => self.is_legal_move_rook(new_pos, board),
                PieceTypes::Queen => self.is_legal_move_queen(new_pos, board),
                PieceTypes::King => self.is_legal_move_king(new_pos, board)
            }
        }

        fn is_legal_move_pawn(&self, new_pos: &Position, board: &Board) -> bool {
            let (dist_x, mut dist_y) = self.position.get_distance_between(new_pos);
            let mut start_pos = 1;
            if let Color::Black = self.color {
                dist_y *= -1;
                start_pos = 6;
            }

            if dist_y == 2 && dist_x == 0 {
                if self.position.y != start_pos {
                    return false;
                }

                //if let None = board
            } else if dist_y == 1 && dist_x.abs() == 1 {

            } else if dist_y == 1 && dist_x == 0 {

            }

            false
        }
        
        fn is_legal_move_knight(&self, new_pos: &Position, board: &Board) -> bool {
            false
        }

        fn is_legal_move_bishop(&self, new_pos: &Position, board: &Board) -> bool {
            false
        }

        fn is_legal_move_rook(&self, new_pos: &Position, board: &Board) -> bool {
            false
        }

        fn is_legal_move_queen(&self, new_pos: &Position, board: &Board) -> bool {
            false
        }

        fn is_legal_move_king(&self, new_pos: &Position, board: &Board) -> bool {
            false
        }
    }
    
    impl fmt::Display for Piece {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.character)
        }
    }
    
    impl fmt::Debug for Piece {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} @ {}/{}", self.character, self.position.x, self.position.y)
        }
    }
    
    const CHAR_SET: [&str; 12] = [
        "♟", "♞", "♝", "♜", "♛", "♚",
        "♙", "♘", "♗", "♖", "♕", "♔"
    ];

    #[derive(Debug, Clone, Copy)]
    pub enum PieceTypes {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
    
    impl PieceTypes {
        pub fn new(&self, color: Color, position: Position) -> Piece {
            let offset = match color {
                Color::White => 0,
                Color::Black => 6
            };
            let taken = false;

            match self {
                PieceTypes::Pawn => Piece {color, position, character: CHAR_SET[offset], taken, piece_type: PieceTypes::Pawn},
                PieceTypes::Knight => Piece {color, position, character: CHAR_SET[1 + offset], taken, piece_type: PieceTypes::Knight},
                PieceTypes::Bishop => Piece {color, position, character: CHAR_SET[2 + offset], taken, piece_type: PieceTypes::Bishop},
                PieceTypes::Rook => Piece {color, position, character: CHAR_SET[3 + offset], taken, piece_type: PieceTypes::Rook},
                PieceTypes::Queen => Piece {color, position, character: CHAR_SET[4 + offset], taken, piece_type: PieceTypes::Queen},
                PieceTypes::King => Piece {color, position, character: CHAR_SET[5 + offset], taken, piece_type: PieceTypes::King}
            }
        }
    }
}