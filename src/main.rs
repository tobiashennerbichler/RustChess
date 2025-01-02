mod piece;
mod board;
mod parser;
mod player;
mod game;
mod tests;

use std::env;

use game::game::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut game;
    
    if args.len() == 1 {
        game = Game::new();
    } else if args.len() == 2 {
        game = match Game::new_from_fen(&args[1]) {
            Ok(game) => game,
            Err(err) => {
                println!("{err}");
                return ();
            }
        }
    } else {
        println!("Invalid number of arguments");
        return ();        
    }
    
    game.run();
}
