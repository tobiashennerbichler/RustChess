pub mod board {
    use crate::piece::piece::{Position, Color, Piece, PieceTypes};
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
                for (piece_index, piece) in player.get_pieces().iter().enumerate() {
                    let pos = piece.get_position();
                    self.grid[pos.x][pos.y] = Some(BoardEntry {player_color: player.get_color(), piece_index});
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
        
        pub fn validate_move(&mut self, player: &Player, notation: &mut ParsedNotation) -> Result<(), &'static str> {
            match *notation {
                ParsedNotation::Short(to, piece_type) => {
                    *notation = self.check_and_convert_short(player, to, piece_type)?;
                    Ok(())
                },
                ParsedNotation::Full(from, to, piece_type) => {
                    self.check_full(player, from, to, piece_type)
                }
            }
        }

        pub fn check_and_convert_short(&self, player: &Player, to: Position, piece_type: PieceTypes) -> Result<ParsedNotation, &'static str> {
            let mut matches: u32 = 0;
            let mut from = Position {x: 0, y: 0};
            for piece in player.get_pieces() {
                if piece.get_piece_type() != piece_type {
                    continue;
                }

                if let Ok(_) = piece.is_possible_move(player, to, self) {
                    from = piece.get_position();
                    matches += 1;
                }
            }
                    
            match matches {
                0 => Err("No match for given piece type"),
                1 => Ok(ParsedNotation::Full(from, to, piece_type)),
                2.. => Err("Multiple possible moves, use full notation [src:dest]")
            }
        }

        pub fn check_full(&self, player: &Player, to: Position, from: Position, piece_type: PieceTypes) -> Result<(), &'static str> {
            let Some(entry) = self.get_board_entry(from) else {
                return Err("Selected field empty");
            };
                    
            if player.get_color() != entry.player_color {
                return Err("Piece on selected field does not belong to the player");
            }

            let piece = player.get_piece(entry.piece_index);
            if piece.get_piece_type() != piece_type {
                return Err("Selected piece does not match piece at selected field");
            }

            piece.is_possible_move(player, to, self)
        }
        
        pub fn execute_or_reset(&mut self, player: &mut Player, enemy: &mut Player, from: Position, to: Position) -> Result<(), &'static str> {
            let already_check = player.is_in_check();

            // Save board state
            let src_entry = self.get_board_entry(from).expect("Must contain piece");
            let opt_dest_entry = self.get_board_entry(to);
            self.execute_move(player, enemy, from, to);

            // If move caused check or did not stop it, reset move
            if player.does_piece_give_check(enemy, self) {
                if let Some(dest_entry) = opt_dest_entry {
                    enemy.untake_piece(dest_entry.piece_index, to);
                }

                player.update_piece_position(src_entry.piece_index, from);
                self.grid[to.x][to.y] = opt_dest_entry;
                self.grid[from.x][from.y] = Some(src_entry);

                return match already_check {
                    true => Err("Currently in check! Cannot move other piece unless it stops check!"),
                    false => Err("Move causes check!")
                }
            }

            Ok(())
        }

        pub fn execute_move(&mut self, player: &mut Player, enemy: &mut Player, from: Position, to: Position) {
            if let Some(dest_entry) = self.get_board_entry(to) {
                enemy.take_piece(dest_entry.piece_index);
            }
            
            let src_entry = self.get_board_entry(from).expect("src field must contain piece");
            player.update_piece_position(src_entry.piece_index, to);
            self.grid[from.x][from.y] = None;
            self.grid[to.x][to.y] = Some(src_entry);
        }
        
        
        pub fn get_board_entry(&self, pos: Position) -> Option<BoardEntry> {
            self.grid[pos.x][pos.y]
        }
    }
}