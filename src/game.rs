pub mod game {
    use std::io;
    use std::io::Write;

    use crate::piece::piece::{Color, Position, PieceTypes};
    use crate::board::board::{Board, BoardEntry};
    use crate::player::player::Player;
    use crate::parser::notation_parser::{parse_fen, parse_action, Action, ParsedFen, ParsedNotation, ParsingErr};

    type GameResult<T> = Result<T, &'static str>;

    pub struct Game {
        board: Board,
        players: [Player; 2],
        current_player: Color
    }

    enum GameState {
        Checkmate,
        Stalemate,
        Ongoing
    }

    impl Game {
        pub fn new() -> Self {
            let mut board = Board::new();
            let white = Player::new(Color::White);
            let black = Player::new(Color::Black);
            let players = [white, black];
            board.init_grid(&players);
            Game {board, players, current_player: Color::White}
        }

        pub fn new_from_fen(fen: &str) -> Result<Self, ParsingErr> {
            let mut board = Board::new();
            let ParsedFen(player_pieces, beginning_player) = parse_fen(fen)?;
            let white = Player::new_from(player_pieces[0].to_owned(), Color::White);
            let black = Player::new_from(player_pieces[1].to_owned(), Color::Black);
            let players = [white, black];
            board.init_grid(&players);

            Ok(Game {board, players, current_player: beginning_player})
        }
        
        pub fn run(&mut self) {
            let mut game_state = self.update_game_state();
            while let GameState::Ongoing = game_state {
                self.board.print(&self.players);

                let Ok(buffer) = read_input() else {
                    println!("Error with reading - try again");
                    continue;
                };

                let action = match parse_action(&buffer) {
                        Ok(action) => action,
                    Err(err) => {
                        println!("{err}");
                        continue;
                    }
                };

                match action {
                    Action::List => {
                        self.list_current_pieces();
                        continue;
                    }
                    Action::Move(notation) => {
                        if let Err(message) = self.validate_and_execute_move(notation) {
                            println!("Invalid move: {message}");
                            continue;
                        }
                    },
                    Action::Quit => return ()
                }
                
                self.current_player = match self.current_player {
                    Color::White => Color::Black,
                    Color::Black => Color::White
                };
                game_state = self.update_game_state();
            }
            
            match game_state {
                GameState::Checkmate => println!("Checkmate! {} lost!", self.current_player),
                GameState::Stalemate => println!("Stalemate!"),
                GameState::Ongoing => panic!("Cannot be ongoing")
            };
        }

        pub fn export_fen(&self) -> String {
            let mut fen = String::new();
            for y in (0..=7).rev() {
                let mut empty = 0;
                for x in 0..=7 {
                    match self.board.get_board_entry(Position {x, y}) {
                        Some(entry) => {
                            if empty > 0 {
                                fen.push(char::from_digit(empty, 10).unwrap());
                                empty = 0;
                            }

                            let piece = match entry.player_color {
                                Color::White => self.players[0].get_piece(entry.piece_index),
                                Color::Black => self.players[1].get_piece(entry.piece_index),
                            };
                                
                            let mut character = match piece.get_piece_type() {
                                PieceTypes::Pawn => 'P',
                                PieceTypes::Knight => 'N',
                                PieceTypes::Bishop => 'B',
                                PieceTypes::Rook => 'R',
                                PieceTypes::Queen => 'Q',
                                PieceTypes::King => 'K'
                            };

                            if entry.player_color == Color::Black {
                                character.make_ascii_lowercase();
                            }
                            fen.push(character);
                        },
                        None => empty += 1
                    }
                }
                if empty > 0 {
                    fen.push(char::from_digit(empty, 10).unwrap());
                }

                match y {
                    0 => fen.push(' '),
                    _ => fen.push('/')
                }                
            }
                
            fen.push_str(self.current_player.into());
            fen
        }
        
        fn list_current_pieces(&mut self) {
            let (player, _) = self.get_ref_players(self.current_player);

            let len = player.get_pieces().len();
            for indx in 0..len {
                let (player, _) = self.get_ref_players(self.current_player);
                let &piece = player.get_piece(indx);
                if piece.is_taken() {
                    continue;
                }

                println!("{piece:?} has possible moves: {:?}", piece.get_legal_positions(self));
            }
        }
        
        pub fn get_all_current_legal_positions(&mut self) -> Vec<Vec<Position>> {
            let (player, _) = self.get_ref_players(self.current_player);
            let mut all_positions = Vec::new();

            let len = player.get_pieces().len();
            for indx in 0..len {
                let (player, _) = self.get_ref_players(self.current_player);
                let &piece = player.get_piece(indx);
                if piece.is_taken() {
                    continue;
                }

                all_positions.push(piece.get_legal_positions(self));
            }

            all_positions
        }

        pub fn validate_and_execute_move(&mut self, notation: ParsedNotation) -> GameResult<()> {
            let (from, to) = match notation {
                ParsedNotation::Short(to, piece_type) => {
                    let from = self.validate_and_convert_short_notation(to, piece_type)?;
                    (from, to)
                },
                ParsedNotation::Full(from, to, piece_type) => {
                    self.validate_full_notation(from, to, piece_type)?;
                    (from, to)
                }
            };
            
            self.does_move_cause_check(from, to)?;
            self.execute_move(from, to);
            Ok(())
        }

        fn validate_and_convert_short_notation(&self, to: Position, piece_type: PieceTypes) -> GameResult<Position> {
            let (player, _) = self.get_ref_players(self.current_player);

            let mut matches: u32 = 0;
            let mut from = Position {x: 0, y: 0};
            for piece in player.get_pieces() {
                if piece.is_taken() || piece.get_piece_type() != piece_type {
                    continue;
                }

                if let Ok(_) = piece.is_field_reachable(to, &self.board) {
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

        fn validate_full_notation(&self, from: Position, to: Position, piece_type: PieceTypes) -> GameResult<()> {
            let (player, _) = self.get_ref_players(self.current_player);

            let Some(entry) = self.board.get_board_entry(from) else {
                return Err("Selected field empty");
            };
                        
            if player.get_color() != entry.player_color {
                return Err("Piece on selected field does not belong to the player");
            }

            let piece = player.get_piece(entry.piece_index);
            if piece.get_piece_type() != piece_type {
                return Err("Selected piece does not match piece at selected field");
            }

            piece.is_field_reachable(to, &self.board)
        }

        pub fn does_move_cause_check(&mut self, from: Position, to: Position) -> GameResult<()> {
            let (player, enemy) = get_mut_ref_players(&mut self.players, self.current_player);

            let already_in_check = player.is_in_check();

            let src_entry = self.board.get_board_entry(from).expect("Must contain piece");
            let opt_dest_entry = self.board.get_board_entry(to);
               
            let now_in_check = self.execute_move(from, to);
            self.revert_move(from, to, src_entry, opt_dest_entry);
                
            if now_in_check {
                return match already_in_check {
                    true => Err("Currently in check! Cannot move piece unless it stops check!"),
                    false => Err("Move causes check!")
                }
            }
            Ok(())
        }

        fn execute_move(&mut self, from: Position, to: Position) -> bool {
            let (player, enemy) = get_mut_ref_players(&mut self.players, self.current_player);
            
            if let Some(dest_entry) = self.board.get_board_entry(to) {
                enemy.take_piece(dest_entry.piece_index);
            }
                    
            let src_entry = self.board.get_board_entry(from).expect("src field must contain piece");
            player.update_piece_position(src_entry.piece_index, to);
            self.board.set_board_entry(from, None);
            self.board.set_board_entry(to, Some(src_entry));
            
            player.gets_checked_by(enemy, &mut self.board)
        }
            
        fn revert_move(&mut self, from: Position, to: Position, src_entry: BoardEntry, opt_dest_entry: Option<BoardEntry>) {
            let (player, enemy) = get_mut_ref_players(&mut self.players, self.current_player);

            if let Some(dest_entry) = opt_dest_entry {
                enemy.untake_piece(dest_entry.piece_index, to);
            }
                    
            player.update_piece_position(src_entry.piece_index, from);
            self.board.set_board_entry(from, Some(src_entry));
            self.board.set_board_entry(to, opt_dest_entry);
        }

        fn update_game_state(&mut self) -> GameState {
            let (player, enemy) = get_mut_ref_players(&mut self.players, self.current_player);
            player.update_check(enemy, &self.board);
            let in_check = player.is_in_check();
            let legal_positions: Vec<Position> = self.get_all_current_legal_positions().into_iter().flatten().collect();
            println!("update_game_state legal positions: {legal_positions:?}");

            if legal_positions.len() == 0 {
                match in_check {
                    true => GameState::Checkmate,
                    false => GameState::Stalemate
                }
            } else {
                GameState::Ongoing
            }
        }
    
        pub fn get_ref_players(&self, current_player: Color) -> (&Player, &Player) {
            let (left, right) = self.players.split_at(1);
            match current_player {
                Color::White => (&left[0], &right[0]),
                Color::Black => (&right[0], &left[0])
            }
        }
        
        pub fn get_ref_board(&self) -> &Board {
            &self.board
        }
    }
    
    fn read_input() -> io::Result<String> {
        print!("Enter move: ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }

    fn get_mut_ref_players(players: &mut [Player; 2], current_player: Color) -> (&mut Player, &mut Player) {
        let (left, right) = players.split_at_mut(1);
        match current_player {
            Color::White => (&mut left[0], &mut right[0]),
            Color::Black => (&mut right[0], &mut left[0])
        }
    }
}