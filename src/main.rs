mod game;
mod board;
use game::Game;
use board::{Plr, Board, BoardState};

use std::{
    env,
    error::Error,
};

fn main() {
    let mut args = env::args();
    args.next();
    let depth = if let Some(arg) = args.next() {
        match arg.parse::<u32>() {
            Ok(depth) => depth,
            Err(e) => {
                eprintln!("you need to input a valid Natural Number");
                eprintln!("{}", e.description());
                return
            },
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
        },
    }
}