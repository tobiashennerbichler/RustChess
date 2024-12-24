pub mod notation_parser {
    use std::fmt;
    use crate::piece::piece::{Position, PieceTypes};

    #[derive(Copy, Clone)]
    pub struct ParsingErr {
        pub message: &'static str
    }
    
    impl fmt::Display for ParsingErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Parsing Error: {}", self.message)
        }
    }
    
    pub enum Action {
        List,
        Move(ParsedNotation),
        Quit
    }
    
    #[derive(Debug)]
    pub struct ParsedNotation {
        pub src_pos: Position,
        pub dest_pos: Position,
        pub piece_type: PieceTypes
    }
    
    pub fn parse_action(s: &str) -> Result<Action, ParsingErr> {
        let trimmed = s.trim();
        if trimmed.len() == 0 {
            return Err(ParsingErr {message: "String empty"});
        }
        
        if !trimmed.is_ascii() {
            return Err(ParsingErr {message: "String consists of non-ascii characters"});
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
            _ => return Err(ParsingErr {message: "First char is not a valid figure"})
        };
        
        let offset = if bytes[0] >= b'a' && bytes[0] <= b'h' { 0 } else { 1 };
        match &bytes[offset..] {
            [sx, sy, b':', dx, dy] => {
                let src_pos = parse_notation(*sx, *sy)?;
                let dest_pos = parse_notation(*dx, *dy)?;
                Ok(Action::Move(ParsedNotation {
                    src_pos,
                    dest_pos,
                    piece_type
                }))
            },
            _ => Err(ParsingErr {message: "String not in format [source:dest]"})
        }
    }
    
    fn parse_notation(cx: u8, cy: u8) -> Result<Position, ParsingErr> {
        if cx < b'a' || cx > b'h' {
            return Err(ParsingErr {message: "Invalid x pos"});
        }

        if cy < b'1' || cy > b'8' {
            return Err(ParsingErr {message: "Invalid y pos"});
        }

        let x = (cx - b'a') as usize;
        let y = (cy - b'1') as usize;
        Ok(Position {x, y})
    }
}