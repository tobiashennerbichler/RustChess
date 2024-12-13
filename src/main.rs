mod piece;
mod board;
mod notation;

use std::io;
use std::io::Write;
use std::fmt;

use piece::piece::{Color, Position, Piece, PieceTypes};
use board::board::Board;
use notation::notation_parser::parse_notation;

fn init_pieces(color: Color) -> Vec<Piece> {
    let mut pieces = Vec::<Piece>::new();
    let mut y = match color {
        Color::White => 1,
        Color::Black => 6
    };
            
    for i in 0..8 {
        pieces.push(PieceTypes::Pawn.new(color, Position {x: i, y}));
    }
            
    match color {
        Color::White => y -= 1,
        Color::Black => y += 1
    };
    for i in 0..2 {
        pieces.push(PieceTypes::Knight.new(color, Position {x: 1+i*5, y}));
        pieces.push(PieceTypes::Bishop.new(color, Position {x: 2+i*3, y}));
        pieces.push(PieceTypes::Rook.new(color, Position {x: 0+i*7, y}));
    }
           
    pieces.push(PieceTypes::Queen.new(color, Position {x: 3, y}));
    pieces.push(PieceTypes::King.new(color, Position {x: 4, y}));
            
    pieces
}

fn read_input() -> io::Result<String> {
    print!("Enter move: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn main() -> io::Result<()> {
    let mut pieces = Vec::<Piece>::new();
    pieces.append(&mut init_pieces(Color::White));
    pieces.append(&mut init_pieces(Color::Black));
    let mut board = Board::new();
    
    board.init(&pieces);
    board.init(&pieces);
    board.print(&pieces);
    
    let mut player = 0;
    let mut game_over = false;
    while(!game_over) {
        let buffer = read_input()?;
        match parse_notation(&buffer) {
            Ok(parsed) => println!("{:?}", parsed),
            Err(err) => println!("{}", err.message)
        };
    }

    Ok(())
}
