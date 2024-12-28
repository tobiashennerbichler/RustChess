mod piece;
mod board;
mod parser;
mod player;
mod tests;

use std::io;
use std::io::Write;
use std::env;

use piece::piece::Color;
use board::board::Board;
use parser::notation_parser::{parse_action, parse_fen, Action, ParsedFen, ParsingErr};
use player::player::Player;

fn read_input() -> io::Result<String> {
    print!("Enter move: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn get_mut_player_enemy(players: &mut Vec<Player>, player_index: usize) -> (&mut Player, &mut Player) {
    let (left, right) = players.split_at_mut(1);
    match player_index {
        0 => (&mut left[0], &mut right[0]),
        1 => (&mut right[0], &mut left[0]),
        _ => panic!("Invalid index")
    }
}

fn game_loop(beginning_player: usize, players: &mut Vec<Player>, board: &mut Board) {
    let mut player_index = beginning_player;
    let game_over = false;

    while !game_over {
        board.print(players);

        let (player, enemy) = get_mut_player_enemy(players, player_index);
        player.update_check(enemy, board);
        println!("current player in check: {}", player.is_in_check());

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
                player.list_pieces(enemy, board);
                continue;
            }
            Action::Move(notation) => {
                if let Err(message) = board.validate_and_execute_move(player, enemy, notation) {
                    println!("Invalid move: {message}");
                    continue;
                }
            },
            Action::Quit => return ()
        }
        
        player_index = (player_index + 1) % 2;
    }
}

fn load_state_from_fen(fen: &str, players: &mut Vec<Player>, board: &mut Board) -> Result<usize, ParsingErr> {
    let ParsedFen(player_pieces, beginning_player) = parse_fen(fen)?;

    players.push(Player::new_from(player_pieces[0].to_owned(), Color::White));
    players.push(Player::new_from(player_pieces[1].to_owned(), Color::Black));
    board.init_grid(players);

    Ok(beginning_player)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut players: Vec<Player> = Vec::new();
    let mut board = Board::new();
    let mut beginning_player = 0;

    if args.len() == 1 {
        players.push(Player::new(Color::White));
        players.push(Player::new(Color::Black));

        board.init_grid(&players);
    } else if args.len() == 2 {
        beginning_player = match load_state_from_fen(&args[1], &mut players, &mut board) {
            Ok(beginning_player) => beginning_player,
            Err(err) => {
                println!("{err}");
                return ();
            }
        };
    } else {
        println!("Invalid number of arguments");
        return ();        
    }

    game_loop(beginning_player, &mut players, &mut board);
}
