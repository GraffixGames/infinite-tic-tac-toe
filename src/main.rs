mod board;
mod game;
use board::Board;
use game::Game;

use std::env;

macro_rules! USAGE_FMT {
    () => {
r#"
Usage:
  {exe_path} [DEPTH]
  DEPTH must be a natural number, and is 0 by default
"#;
    }
}

fn main() {
    let mut args = env::args();
    let exe_path = args.next().unwrap();
    let depth = if let Some(arg) = args.next() {
        match arg.parse::<u8>() {
            Ok(depth) => depth,
            Err(_) => {
                eprint!(USAGE_FMT!(), exe_path = exe_path);
                return;
            }
        }
    } else {
        0
    };

    let mut game = Game::new(depth);

    while game.winner().is_none() {
        game.play_turn();
    }

    match game.winner().unwrap() {
        None => println!("Nobody won, it was a tie!"),
        Some(p) => {
            let player: &str = p.into();
            println!("{} won the game!", player);
        }
    }
}
