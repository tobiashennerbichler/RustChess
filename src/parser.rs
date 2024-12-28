pub mod notation_parser {
    use std::fmt;
    use crate::piece::piece::{Position, Piece, PieceTypes, Color};

    #[derive(Copy, Clone)]
    pub struct ParsingErr(&'static str);
    
    impl fmt::Display for ParsingErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parsing Error: {}", self.0)
        }
    }
    
    pub enum Action {
        List,
        Move(ParsedNotation),
        Quit
    }
    
    #[derive(Debug)]
    pub enum ParsedNotation {
        Short(Position, PieceTypes),
        Full(Position, Position, PieceTypes)
    }
    
    pub struct ParsedFen(pub [Vec<Piece>; 2], pub usize);
    
    pub fn parse_action(s: &str) -> Result<Action, ParsingErr> {
        let trimmed = s.trim();
        if trimmed.len() == 0 {
            return Err(ParsingErr("String empty"));
        }
        
        if !trimmed.is_ascii() {
            return Err(ParsingErr("String consists of non-ascii characters"));
        }

        match trimmed {
            "l" | "list" => Ok(Action::List),
            "q" | "quit" => Ok(Action::Quit),
            _ => parse_move(trimmed)
        }
    }
    
    fn parse_move(s: &str) -> Result<Action, ParsingErr> {
        let bytes = s.as_bytes();
        let piece_type = match bytes[0] {
            b'N' => PieceTypes::Knight,
            b'B' => PieceTypes::Bishop,
            b'R' => PieceTypes::Rook,
            b'Q' => PieceTypes::Queen,
            b'K' => PieceTypes::King,
            b'P' | b'a'..=b'h' => PieceTypes::Pawn,
            _ => return Err(ParsingErr("First char is not a valid figure"))
        };
        
        let offset = if bytes[0] >= b'a' && bytes[0] <= b'h' { 0 } else { 1 };
        match &bytes[offset..] {
            [dx, dy] => {
                let to = parse_notation(*dx, *dy)?;
                Ok(Action::Move(ParsedNotation::Short(to, piece_type)))
            },
            [sx, sy, b':', dx, dy] => {
                let from= parse_notation(*sx, *sy)?;
                let to= parse_notation(*dx, *dy)?;
                Ok(Action::Move(ParsedNotation::Full(from, to, piece_type)))
            },
            _ => Err(ParsingErr("String not in format [source:dest]"))
        }
    }
    
    fn parse_notation(cx: u8, cy: u8) -> Result<Position, ParsingErr> {
        if cx < b'a' || cx > b'h' {
            return Err(ParsingErr("Invalid x pos"));
        }

        if cy < b'1' || cy > b'8' {
            return Err(ParsingErr("Invalid y pos"));
        }

        let x = (cx - b'a') as usize;
        let y = (cy - b'1') as usize;
        Ok(Position {x, y})
    }

    const FEN_ADDONS_IMPLEMENTED: usize = 1;
    pub fn parse_fen(fen: &str) -> Result<ParsedFen, ParsingErr> {
        let trimmed = fen.trim();
        let mut ranks: Vec<&str> = trimmed.split("/").collect();
        if ranks.len() != 8 {
            return Err(ParsingErr("FEN string must encode all 8 ranks"));
        }

        let remaining: Vec<&str> = ranks[7].split(" ").collect();
        if remaining.len() != FEN_ADDONS_IMPLEMENTED + 1 {
            return Err(ParsingErr("FEN string must contain additional information about board state"));
        }
        ranks[7] = remaining[0];

        let mut pieces: [Vec<Piece>; 2] = [Vec::new(), Vec::new()];
        let mut curr_pos = Position {x: 0, y: 8};
        for rank in ranks {
            curr_pos = Position {x: 0, y: curr_pos.y - 1};
            for c in rank.chars() {
                let color = if c.is_lowercase() { Color::Black } else { Color::White };
                match c {
                    'p' | 'P' => pieces[color as usize].push(Piece::new(PieceTypes::Pawn, color, curr_pos)),
                    'n' | 'N' => pieces[color as usize].push(Piece::new(PieceTypes::Knight, color, curr_pos)),
                    'b' | 'B' => pieces[color as usize].push(Piece::new(PieceTypes::Bishop, color, curr_pos)),
                    'r' | 'R' => pieces[color as usize].push(Piece::new(PieceTypes::Rook, color, curr_pos)),
                    'q' | 'Q' => pieces[color as usize].push(Piece::new(PieceTypes::Queen, color, curr_pos)),
                    'k' | 'K' => pieces[color as usize].push(Piece::new(PieceTypes::King, color, curr_pos)),
                    x @ '1'..='8' => {
                        let empty = x.to_digit(10).unwrap();
                        curr_pos.x += empty as usize - 1;
                    },
                    _ => return Err(ParsingErr("Invalid character"))
                }
                curr_pos.x += 1;
            }

            if curr_pos.x != 8 {
                return Err(ParsingErr("Must notate all 8 fields per rank"));
            }
        }
        
        let num_white_kings = pieces[0].iter().filter(|&p| p.get_piece_type() == PieceTypes::King).count();
        let num_black_kings = pieces[1].iter().filter(|&p| p.get_piece_type() == PieceTypes::King).count();
        if num_white_kings + num_black_kings != 2 {
            return Err(ParsingErr("Incorrect number of kings"));
        }
        
        let beginning_player = match remaining[1] {
            "w" => 0,
            "b" => 1,
            _ => return Err(ParsingErr("Incorrect beginning player"))
        };
            
        Ok(ParsedFen(pieces, beginning_player))
    }
}