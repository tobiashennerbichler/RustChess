pub mod player {
    use crate::piece::piece::{Piece, PieceTypes, Position, Color};
    use crate::board::board::Board;
    
    pub struct Player {
        pieces: Vec<Piece>,
        color: Color,
        in_check: bool
    }

    fn init_pieces(color: Color) -> Vec<Piece> {
        let mut pieces = Vec::new();
        let mut y = match color {
            Color::White => 1,
            Color::Black => 6
        };
                        
        for i in 0..8 {
            pieces.push(Piece::new(PieceTypes::Pawn, color, Position {x: i, y}));
        }
                        
        match color {
            Color::White => y -= 1,
            Color::Black => y += 1
        };
        for i in 0..2 {
            pieces.push(Piece::new(PieceTypes::Rook, color, Position {x: 0+i*7, y}));
            pieces.push(Piece::new(PieceTypes::Knight, color, Position {x: 1+i*5, y}));
            pieces.push(Piece::new(PieceTypes::Bishop, color, Position {x: 2+i*3, y}));
        }
                       
        pieces.push(Piece::new(PieceTypes::Queen, color, Position {x: 3, y}));
        pieces.push(Piece::new(PieceTypes::King, color, Position {x: 4, y}));
        pieces
    }
    
    impl Player {
        pub fn new(color: Color) -> Self {
            let pieces = init_pieces(color);
            Player { pieces, color, in_check: false }
        }
        
        pub fn get_pieces(&self) -> &Vec<Piece> {
            &self.pieces
        }

        pub fn get_piece(&self, index: usize) -> &Piece {
            &self.pieces[index]
        }

        pub fn take_piece(&mut self, piece_index: usize) {
            if self.pieces[piece_index].is_taken() {
                panic!("Piece should not be taken already");
            }

            self.pieces[piece_index].take();
        }

        pub fn untake_piece(&mut self, piece_index: usize, pos: Position) {
            if !self.pieces[piece_index].is_taken() {
                panic!("Piece should be taken");
            }

            self.pieces[piece_index].untake(pos);
        }

        pub fn update_piece_position(&mut self, piece_index: usize, new_pos: Position) {
            self.pieces[piece_index].update_position(new_pos);
        }

        pub fn list_pieces(&self) {
            for &piece in self.pieces.iter() {
                println!("{piece:?}");
            }
        }
        
        pub fn gets_checked_by(&self, enemy: &Player, board: &Board) -> bool {
            let king: &Piece = self.pieces.iter().filter(|&&p| p.get_piece_type() == PieceTypes::King)
                                                 .collect::<Vec<&Piece>>()[0];
            
            for piece in enemy.get_pieces() {
                if piece.is_taken() {
                    continue;
                }

                if let Ok(_) = piece.is_possible_move(enemy, king.get_position(), board) {
                    println!("This {piece:?} piece gives check to {king:?}");
                    return true;
                }
            }
            false
        }

        pub fn update_check(&mut self, enemy: &Player, board: &Board) {
            self.in_check = self.gets_checked_by(enemy, board);
        }

        pub fn get_color(&self) -> Color {
            self.color
        }

        pub fn is_in_check(&self) -> bool {
            self.in_check
        }
    }

}
