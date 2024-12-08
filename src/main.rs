#[derive(Debug, Copy, Clone)]
enum Color {
    White,
    Black
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black")
        }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct CommonPiece {
    color: Color,
    position: Position,
    character: &'static str,
    taken: bool
}

impl CommonPiece {
    fn new(color: Color, position: Position, character: &'static str) -> Self {
        CommonPiece {
            color,
            position,
            character,
            taken: false
        }
    }
}

// FIXME: one new that can make each piece with correct unicode character
enum Piece {
    Pawn(CommonPiece),
    Knight(CommonPiece),
    Bishop(CommonPiece),
    Rook(CommonPiece),
    Queen(CommonPiece),
    King(CommonPiece)
}

impl Piece {
    fn move_pawn(&self, new_pos: Position) -> bool {
        true
    }
    
    fn move_knight(&self, new_pos: Position) -> bool {
        true
    }
}

use std::fmt;
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(piece) | 
            Piece::Rook(piece) |
            Piece::Knight(piece) |
            Piece::Bishop(piece) |
            Piece::Queen(piece) |
            Piece::King(piece) => write!(f, "{}", piece.character)
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(piece) |
            Piece::Rook(piece) |
            Piece::Knight(piece) |
            Piece::Bishop(piece) |
            Piece::Queen(piece) |
            Piece::King(piece) => write!(f, "{} @ {}/{}", piece.character, piece.position.x, piece.position.y)
        }
    }
}

struct Board<'a> {
    grid: [[Option<&'a Piece>; 8]; 8]
}

impl<'a> Board<'a> {
    fn init(&mut self, pieces: &'a Vec<Piece>) {
        for piece in pieces {
            match piece {
                Piece::Pawn(common) |
                Piece::Rook(common) |
                Piece::Knight(common) |
                Piece::Bishop(common) |
                Piece::Queen(common) |
                Piece::King(common) => self.grid[common.position.x][common.position.y] = Some(piece)
            };
        }
    }
    
    fn print(&self) {
        for y in 0..8usize {
            for x in 0..8usize {
                match self.grid[x][y] {
                    Some(piece) => print!("{piece} "),
                    None => print!("- ")
                };
            }
            println!("");
        }
    }
}

const CHAR_SET: [&'static str; 12] = [
    "♙", "♖", "♘", "♗", "♕", "♔",
    "♟", "♜", "♞", "♝", "♛", "♚"
];

fn init_pieces(color: Color) -> Vec<Piece> {
    let mut pieces = Vec::<Piece>::new();
    //FIXME: figure out nice way to do offsets
    let (y1, y2, char_set_offset) = match color {
        Color::White => (0, 1, 0),
        Color::Black => (7, 6, 6)
    };
    
    for i in 0..8 {
        pieces.push(Piece::Pawn(CommonPiece::new(color, Position {x: i, y: y2}, CHAR_SET[char_set_offset])));
    }
    
    for i in 0..2 {
        pieces.push(Piece::Rook(CommonPiece::new(color, Position {x: 0+i*7, y: y1}, CHAR_SET[1 + char_set_offset])));
        pieces.push(Piece::Knight(CommonPiece::new(color, Position {x: 1+i*5, y: y1}, CHAR_SET[2 + char_set_offset])));
        pieces.push(Piece::Bishop(CommonPiece::new(color, Position {x: 2+i*3, y: y1}, CHAR_SET[3 + char_set_offset])));
    }
    
    pieces.push(Piece::Queen(CommonPiece::new(color, Position {x: 3, y: y1}, CHAR_SET[4 + char_set_offset])));
    pieces.push(Piece::King(CommonPiece::new(color, Position {x: 4, y: y1}, CHAR_SET[5 + char_set_offset])));
    
    pieces
}

fn main() {
    let white_pieces = init_pieces(Color::White);
    let black_pieces = init_pieces(Color::Black);
    let mut board = Board {
        grid: [[None; 8]; 8]
    };
    
    board.init(&white_pieces);
    board.init(&black_pieces);
    board.print();
}
