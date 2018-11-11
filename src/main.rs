mod game;
mod board;
use game::Game;
use board::{Board, Winner, BoardState};

use std::{
    env,
    error::Error,
};

const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    X,
    O,
}

impl Tile {
    pub fn swap(self) -> Self {
        match self {
            Tile::X => Tile::O,
            Tile::O => Tile::X,
        }
    }
}

impl<'a> Into<&'a str> for Tile {
    fn into(self) -> &'a str {
        match self {
            Tile::X => "X",
            Tile::O => "O",
        }
    }
}

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
        Winner::Tie => println!("Nobody won, it was a tie!"),
        Winner::Tile(t) => {
            let tile: &str = t.into();
            println!("{} won the game!", tile);
        },
    }
}