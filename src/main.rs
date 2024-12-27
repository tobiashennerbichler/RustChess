mod piece;
mod board;
mod parser;
mod player;

use std::io;
use std::io::Write;
use std::env;

use piece::piece::Color;
use board::board::Board;
use parser::notation_parser::{parse_action, Action};
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

fn game_loop(players: &mut Vec<Player>, board: &mut Board) {
    let mut player_index = 0;
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
                player.list_pieces(board);
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

fn main() {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new(Color::White));
    players.push(Player::new(Color::Black));

    let mut board = Board::new();
    board.init_grid(&players);

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- [headless/gui]");
        return ()
    }

    match args[1].as_str() {
        "headless" => game_loop(&mut players, &mut board),
        "gui" => panic!("Not implemented yet"),
        _ => println!("Usage: cargo run -- [headless/gui]")
    }
}
