mod piece;
mod board;
mod parser;
mod player;

use std::io;
use std::io::Write;

use piece::piece::Color;
use board::board::Board;
use parser::notation_parser::parse_notation;
use player::player::Player;

fn read_input() -> io::Result<String> {
    print!("Enter move: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn game_loop(players: &mut Vec<Player>, board: &mut Board) -> io::Result<()> {
    let mut player_index = 0;
    let mut game_over = false;
    while !game_over {
        // TODO: update checks
        board.print(players);
        let buffer = read_input()?;
        let parsed = match parse_notation(&buffer) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        
        let player = &players[player_index];
        if let Err(message) = board.is_valid_move(player, &parsed) {
            println!("Incorrect move: {message}");
            continue;
        }

        board.execute_move(players, player_index, &parsed);
        player_index = (player_index + 1) % 2;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new(Color::White));
    players.push(Player::new(Color::Black));

    let mut board = Board::new();
    board.init_grid(&players);
    game_loop(&mut players, &mut board)?;
    
    Ok(())
}
