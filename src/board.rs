pub mod board {
    use crate::piece::piece::{Position, Color};
    use crate::player::player::Player;

    #[derive(Copy, Clone)]
    pub struct BoardEntry {
        pub player_color: Color,
        pub piece_index: usize
    }
    
    pub struct Board {
        grid: [[Option<BoardEntry>; 8]; 8]
    }
    
    impl Board {
        pub fn new() -> Self {
            Board { grid: [[None; 8]; 8] }
        }
        
        pub fn init_grid(&mut self, players: &[Player; 2]) {
            for player in players {
                for (piece_index, piece) in player.get_pieces().iter().enumerate() {
                    let pos = piece.get_position();
                    self.grid[pos.x][pos.y] = Some(BoardEntry {player_color: player.get_color(), piece_index});
                }
            }
        }
        
        pub fn print(&self, players: &[Player; 2]) {
            if players[1].is_in_check() {
                println!("Check!");
            }

            for y in (0..8usize).rev() {
                print!("{} ", y + 1);
                for x in 0..8usize {
                    match self.grid[x][y] {
                        Some(entry) => {
                            let player_index = entry.player_color as usize;
                            print!("{} ", players[player_index].get_piece(entry.piece_index));
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

            match players[0].is_in_check() {
                true => println!("Check!"),
                false => println!("")
            }
        }
        
        pub fn get_board_entry(&self, pos: Position) -> Option<BoardEntry> {
            self.grid[pos.x][pos.y]
        }
        
        pub fn set_board_entry(&mut self, pos: Position, entry: Option<BoardEntry>) {
            self.grid[pos.x][pos.y] = entry;
        }
    }
}