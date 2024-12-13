pub mod board {
    use crate::piece::piece::{Piece, Position};
    pub struct Board {
        grid: [[Option<usize>; 8]; 8]
    }
    
    impl Board {
        pub fn new() -> Self {
            Board { grid: [[None; 8]; 8] }
        }

        pub fn init(&mut self, pieces: &Vec<Piece>) {
            for (indx, piece) in pieces.into_iter().enumerate() {
                let pos = piece.get_position();
                self.grid[pos.x][pos.y] = Some(indx);
            };
        }
        
        pub fn print(&self, pieces: &Vec<Piece>) {
            for y in (0..8usize).rev() {
                print!("{y} ");
                for x in (0..8usize).rev() {
                    match self.grid[x][y] {
                        Some(indx) => {
                            print!("{} ", pieces[indx]);
                        },
                        None => print!("- ")
                    };
                }
                println!("");
            }
            print!("  ");
            for x in 0..8 {
                let character = b'a' + x;
                print!("{} ", character as char);
            }
            println!("");
        }
        
        pub fn is_empty_field(&self, pos: Position) -> bool {
            match self.grid[pos.x][pos.y] {
                Some(_) => false,
                None => true
            }
        }
        
        pub fn get_index(&self, pos: Position) -> Option<usize> {
            self.grid[pos.x][pos.y]
        }
        
    }
}