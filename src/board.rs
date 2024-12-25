pub mod board {
    use crate::piece::piece::{Position, Color};
    use crate::player::player::Player;
    use crate::parser::notation_parser::ParsedNotation;

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
        
        pub fn init_grid(&mut self, players: &Vec<Player>) {
            for player in players {
                for (piece_index, piece) in player.pieces.iter().enumerate() {
                    let pos = piece.get_position();
                    self.grid[pos.x][pos.y] = Some(BoardEntry {player_color: player.color, piece_index});
                }
            }
        }
        
        pub fn print(&self, players: &Vec<Player>) {
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
            println!("");
        }
        
        pub fn is_valid_move(&self, player: &Player, notation: &mut ParsedNotation) -> Result<(), &'static str> {
            match *notation {
                ParsedNotation::ShortNotation(to, piece_type) => {
                    let mut matches: u32 = 0;
                    let mut from = Position {x: 0, y: 0};
                    for piece in player.get_pieces() {
                        if piece.get_piece_type() != piece_type {
                            continue;
                        }

                        if let Ok(_) = piece.is_legal_move(player, to, self) {
                            from = piece.get_position();
                            matches += 1;
                        }
                    }
                    
                    match matches {
                        0 => Err("No match for given piece type"),
                        1 => {
                            *notation = ParsedNotation::FullNotation(from, to, piece_type);
                            Ok(())
                        },
                        2.. => Err("Multiple possible moves, use full notation [src:dest]")
                    }
                },
                ParsedNotation::FullNotation(from, to, piece_type) => {
                    let Some(entry) = self.get_board_entry(from) else {
                        return Err("Selected field empty");
                    };
                    
                    if player.color != entry.player_color {
                        return Err("Piece on selected field does not belong to the player");
                    }

                	let piece = player.get_piece(entry.piece_index);
                	if piece.get_piece_type() != piece_type {
                	    return Err("Selected piece does not match piece at selected field");
                	}

                	piece.is_legal_move(player, to, self)
                }
            }
        }

        pub fn execute_move(&mut self, players: &mut Vec<Player>, player_index: usize, from: Position, to: Position) {
            let enemy_index = (player_index + 1) % 2;
            if let Some(dest_entry) = self.get_board_entry(to) {
                players[enemy_index].take_piece(dest_entry.piece_index);
            }
            
            let src_entry = self.get_board_entry(from).expect("Src field should contain piece, check is_legal functions");
            players[player_index].update_piece_position(src_entry.piece_index, to);
            self.grid[from.x][from.y] = None;
            self.grid[to.x][to.y] = Some(src_entry);
        }

        pub fn get_board_entry(&self, pos: Position) -> Option<BoardEntry> {
            self.grid[pos.x][pos.y]
        }
    }
}