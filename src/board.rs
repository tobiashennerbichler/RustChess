pub mod board {
    use crate::piece::piece::{Position, Color, Piece, PieceTypes};
    use crate::player::player::Player;
    use crate::parser::notation_parser::{ParsedNotation};

    type BoardResult<T> = Result<T, &'static str>;

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
        
        pub fn validate_and_execute_move(&mut self, player: &mut Player, enemy: &mut Player, notation: ParsedNotation) -> BoardResult<()> {
            let (from, to) = match notation {
                ParsedNotation::Short(to, piece_type) => {
                    let from = self.validate_and_convert_short_notation(player, to, piece_type)?;
                    (from, to)
                },
                ParsedNotation::Full(from, to, piece_type) => {
                    self.validate_full_notation(player, from, to, piece_type)?;
                    (from, to)
                }
            };
            
            self.does_move_cause_check(player, enemy, from, to)?;
            self.execute_move(player, enemy, from, to);
            Ok(())
        }

        pub fn does_move_cause_check(&mut self, player: &mut Player, enemy: &mut Player, from: Position, to: Position) -> BoardResult<()> {
            let already_in_check = player.is_in_check();

            let src_entry = self.get_board_entry(from).expect("Must contain piece");
            let opt_dest_entry = self.get_board_entry(to);
            
            self.execute_move(player, enemy, from, to);
            let now_in_check = player.gets_checked_by(enemy, self);
            self.revert_move(player, enemy, from, to, src_entry, opt_dest_entry);
            
            if now_in_check {
                return match already_in_check {
                    true => Err("Currently in check! Cannot move piece unless it stops check!"),
                    false => Err("Move causes check!")
                }
            }
            Ok(())
        }

        fn validate_and_convert_short_notation(&self, player: &Player, to: Position, piece_type: PieceTypes) -> BoardResult<Position> {
            let mut matches: u32 = 0;
            let mut from = Position {x: 0, y: 0};
            for piece in player.get_pieces() {
                if piece.is_taken() || piece.get_piece_type() != piece_type {
                    continue;
                }

                if let Ok(_) = piece.is_field_reachable(player, to, self) {
                    from = piece.get_position();
                    matches += 1;
                }
            }
                    
            match matches {
                0 => Err("Cannot find matching piece to move to destination"),
                1 => Ok(from),
                2.. => Err("Multiple possible moves - use full notation: [src:dest]")
            }
        }

        fn validate_full_notation(&self, player: &Player, from: Position, to: Position, piece_type: PieceTypes) -> BoardResult<()> {
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

            piece.is_field_reachable(player, to, self)
        }
        
        fn execute_move(&mut self, player: &mut Player, enemy: &mut Player, from: Position, to: Position) {
            if let Some(dest_entry) = self.get_board_entry(to) {
                enemy.take_piece(dest_entry.piece_index);
            }
            
            let src_entry = self.get_board_entry(from).expect("src field must contain piece");
            player.update_piece_position(src_entry.piece_index, to);
            self.set_board_entry(from, None);
            self.set_board_entry(to, Some(src_entry));
        }
        
        fn revert_move(&mut self, player: &mut Player, enemy: &mut Player, from: Position, to: Position, src_entry: BoardEntry,
            opt_dest_entry: Option<BoardEntry>) {
            if let Some(dest_entry) = opt_dest_entry {
                enemy.untake_piece(dest_entry.piece_index, to);
            }
            
            player.update_piece_position(src_entry.piece_index, from);
            self.set_board_entry(from, Some(src_entry));
            self.set_board_entry(to, opt_dest_entry);
        }

        pub fn get_board_entry(&self, pos: Position) -> Option<BoardEntry> {
            self.grid[pos.x][pos.y]
        }
        
        pub fn set_board_entry(&mut self, pos: Position, entry: Option<BoardEntry>) {
            self.grid[pos.x][pos.y] = entry;
        }
    }
}