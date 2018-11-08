use std::io::{self, Read, Write};

const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InvalidMove;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GameState {
    Winner(Tile),
    Tie,
    Running,
}

#[derive(Clone, Copy, Debug)]
struct Game {
    board: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Tile,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            current_player: Tile::X,
        }
    }

    pub fn prompt_user(&self) -> (usize, usize) {
        loop {
            let player: &str = self.current_player.into();
            print!("{}, please type the location of your move (row, column): ", player);
            io::stdout().flush().expect("could not flush stdout");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("could not read line");
            let (row, col) = {
                let parsed_input: Vec<char> = buffer
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect();
                
                if parsed_input.len() != 2 {
                    println!("your input was not valid, please make sure you only use '1' '2' '3' ',' and ' '");
                    continue;
                }

                let row = parsed_input[0].to_digit(10).unwrap();
                let col = parsed_input[1].to_digit(10).unwrap();
                (row as usize, col as usize)
            };

            if row <= 3 && col <= 3 {
                break (row, col);
            }
        }
    }

    pub fn draw_board(&self) {
        print!("  ");
        for column_marker in 1..=BOARD_SIZE {
            print!("{} ", column_marker);
        }
        println!("");

        for row in 0..self.board.len() {
            print!("{}", row + 1);
            for tile in 0..self.board[row].len() {
                print!(" ");
                let tile = match self.board[row][tile] {
                    Some(t) => t.into(),
                    None => "?",
                };
                print!("{}", tile);
            }
            println!("");
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), InvalidMove> {
        if self.board[row - 1][col-1].is_some() {
            return Err(InvalidMove);
        }
        self.board[row - 1][col - 1] = Some(self.current_player);
        self.current_player = self.current_player.swap();
        Ok(())
    }

    pub fn turn(&mut self) {
        self.draw_board();
        loop {
            let (row, col) = self.prompt_user();
            if self.make_move(row, col) == Ok(()) {
                break;
            }
            println!("That spot is occupied");
        }
    }

    pub fn state(&self) -> GameState {
        let combos = &[
            [self.board[0][0], self.board[0][1], self.board[0][2]],
            [self.board[1][0], self.board[1][1], self.board[1][2]],
            [self.board[2][0], self.board[2][1], self.board[2][2]],

            [self.board[0][0], self.board[1][0], self.board[2][0]],
            [self.board[0][1], self.board[1][1], self.board[2][1]],
            [self.board[0][2], self.board[1][2], self.board[2][2]],

            [self.board[0][0], self.board[1][1], self.board[2][2]],
            [self.board[0][2], self.board[1][1], self.board[2][0]],
        ];

        for combo in combos {
            if let Some(tile) = combo[0] {
                if combo.iter().all(|t| *t == Some(tile)) {
                    return GameState::Winner(tile)
                }
            }
        };

        if combos.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
            GameState::Tie
        } else {
            GameState::Running
        }
    }
}

fn main() {
    let mut game = Game::new();
    while game.state() == GameState::Running {
        game.turn();
    }

    match game.state() {
        GameState::Tie => println!("The game was a tie"),
        GameState::Winner(tile) => {
            let tile: &str = tile.into();
            println!("The winner was: {}", tile);
        },
        _ => ()
    }
}
