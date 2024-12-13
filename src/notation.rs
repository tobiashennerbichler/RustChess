pub mod notation_parser {
    use std::fmt;
    use crate::piece::piece::{Position, PieceTypes};

    #[derive(Copy, Clone)]
    pub struct ParseNotationErr {
        pub message: &'static str
    }
    
    impl fmt::Display for ParseNotationErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid chess notation string: {}", self.message)
        }
    }
    
    #[derive(Debug)]
    pub struct ParsedNotation {
        pub source_pos: Position,
        pub dest_pos: Position,
        pub piece_type: PieceTypes
    }
    
    pub fn parse_notation(s: &str) -> Result<ParsedNotation, ParseNotationErr> {
        let trimmed = s.trim();
        if trimmed.len() == 0 {
            return Err(ParseNotationErr {message: "String empty"});
        }
    
        if !trimmed.is_ascii() {
            return Err(ParseNotationErr {message: "String consists of non-ascii characters"});
        }
    
        let bytes = trimmed.as_bytes();
        let Some(piece_type) = (match bytes[0] {
            b'N' => Some(PieceTypes::Knight),
            b'B' => Some(PieceTypes::Bishop),
            b'R' => Some(PieceTypes::Rook),
            b'Q' => Some(PieceTypes::Queen),
            b'K' => Some(PieceTypes::King),
            b'P' | b'a'..=b'h' => Some(PieceTypes::Pawn),
            _ => None
        }) else {
            return Err(ParseNotationErr {message: "First char is not a valid figure"});
        };
        
        let offset = if bytes[0] >= b'a' && bytes[0] <= b'h' { 0 } else { 1 };
        match &bytes[offset..] {
            [sx, sy, b':', dx, dy] => {
                let source_pos = parse_position(*sx, *sy)?;
                let dest_pos = parse_position(*dx, *dy)?;
                Ok(ParsedNotation {
                    source_pos,
                    dest_pos,
                    piece_type
                })
            },
            _ => Err(ParseNotationErr {message: "String not in format [source:dest]"})
        }
    }
    
    fn parse_position(ux: u8, uy: u8) -> Result<Position, ParseNotationErr> {
        if ux < b'a' || ux > b'h' {
            return Err(ParseNotationErr {message: "Invalid x pos"});
        }
    
        if uy < b'0' || uy > b'9' {
            return Err(ParseNotationErr {message: "Invalid y pos"});
        }
        
        let x = (ux - b'a') as usize;
        let y = (uy - b'0') as usize;
        Ok(Position {x, y})
    }
    
}