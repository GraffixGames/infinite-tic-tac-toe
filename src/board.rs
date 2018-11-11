use std::io::{self, Write};
use super::{Tile, BOARD_SIZE};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardState {
    Complete(Winner),
    Running,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Winner {
    Tile(Tile),
    Tie,
}

#[derive(Clone, Debug)]
pub enum Board {
    Base([[Option<Tile>; BOARD_SIZE]; BOARD_SIZE]),
    Over(Vec<Vec<Box<Board>>>),
}

impl Board {
    pub fn new(sub_boards: u32) -> Self {
        if sub_boards == 0 {
            Board::Base([[None; BOARD_SIZE]; BOARD_SIZE])
        } else {
            Board::Over(vec![vec![Box::new(Board::new(sub_boards - 1)); BOARD_SIZE]; BOARD_SIZE])
        }
    }

    fn draw(&self) {
        match *self {
            Board::Base(ref tiles) => {
                println!("Base Board");
                print!("  ");
                for column_marker in 1..=BOARD_SIZE {
                    print!("{} ", column_marker);
                }
                println!("");

                for row in 0..tiles.len() {
                    print!("{}", row + 1);
                    for col in 0..tiles[row].len() {
                        print!(" ");
                        let tile = match tiles[row][col] {
                            Some(t) => t.into(),
                            None => "?",
                        };
                        print!("{}", tile);
                    }
                    println!("");
                }
            }

            Board::Over(ref boards) => {
                println!("Over Board");
                print!("  ");
                for column_marker in 1..=BOARD_SIZE {
                    print!("{} ", column_marker);
                }
                println!("");

                for row in 0..boards.len() {
                    print!("{}", row + 1);
                    for col in 0..boards[row].len() {
                        print!(" ");
                        let tile = match boards[row][col].state() {
                            BoardState::Running => "?",
                            BoardState::Complete(Winner::Tile(t)) => t.into(),
                            BoardState::Complete(Winner::Tie) => "-",
                        };
                        print!("{}", tile);
                    }
                    println!("");
                }
            }
        }
    }

    pub fn state(&self) -> BoardState {
        let mut combos = Vec::new();
        combos.extend({
            let mut row_combos = Vec::new();
            for row in 0..BOARD_SIZE {
                let mut combo = Vec::new();
                for col in 0..BOARD_SIZE {
                    match *self {
                        Board::Base(ref tiles) => combo.push(tiles[row][col]),
                        Board::Over(ref boards) => combo.push(match boards[row][col].state(){
                            BoardState::Complete(Winner::Tile(t)) => Some(t),
                            _ => None,
                        }),
                    }
                }
                row_combos.push(combo);
            }
            row_combos
        });
        combos.extend({
            let mut column_combos = Vec::new();
            for col in 0..BOARD_SIZE {
                let mut combo = Vec::new();
                for row in 0..BOARD_SIZE {
                    match *self {
                        Board::Base(ref tiles) => combo.push(tiles[row][col]),
                        Board::Over(ref boards) => combo.push(match boards[row][col].state(){
                            BoardState::Complete(Winner::Tile(t)) => Some(t),
                            _ => None,
                        }),
                    }
                }
                column_combos.push(combo);
            }
            column_combos
        });
        combos.extend({
            let mut tl_br = Vec::new();
            let mut tr_bl = Vec::new();
            for coord in 0..BOARD_SIZE {
                match *self {
                    Board::Base(ref tiles) => {
                        tl_br.push(tiles[coord][coord]);
                        tr_bl.push(tiles[coord][BOARD_SIZE - 1 - coord]);
                    },
                    Board::Over(ref boards) => {
                        tl_br.push(match boards[coord][coord].state() {
                            BoardState::Complete(Winner::Tile(t)) => Some(t),
                            _ => None,
                        });
                        tr_bl.push(match boards[coord][BOARD_SIZE - 1 - coord].state() {
                            BoardState::Complete(Winner::Tile(t)) => Some(t),
                            _ => None,
                        });
                    }
                }
            }
            vec![tl_br, tr_bl]
        });

        for combo in &combos {
            if let Some(tile) = combo[0] {
                if combo.iter().all(|t| *t == Some(tile)) {
                    return BoardState::Complete(Winner::Tile(tile))
                }
            }
        };

        if combos.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
            BoardState::Complete(Winner::Tie)
        } else {
            BoardState::Running
        }
    }

    fn make_move(&mut self, player: Tile, row: usize, col: usize) -> Result<(), ()> {
        match *self {
            Board::Base(ref mut tiles) => {
                if tiles[row - 1][col - 1].is_some() {
                    return Err(())
                }
                tiles[row - 1][col - 1] = Some(player);
                Ok(())
            },
            Board::Over(ref mut boards) => {
                if boards[row - 1][col - 1].state() != BoardState::Running {
                    return Err(())
                }
                boards[row - 1][col - 1].play_turn(player);
                Ok(())
            }
        }
        
    }

    pub fn play_turn(&mut self, player: Tile) {
        self.draw();
        loop {
            let (row, col) = prompt(player);
            if self.make_move(player, row, col).is_ok() {
                break;
            }
            println!("That spot is occupied");
        }
    }
}

fn prompt(player: Tile) -> (usize, usize) {
        let player: &str = player.into();

        loop {
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

            if row <= BOARD_SIZE && col <= BOARD_SIZE {
                break (row, col);
            } else {
                println!(
                    "({0}, {1}) is out of range if the board of size ({2}, {2})", 
                    row,
                    col,
                    BOARD_SIZE,
                )
            }
        }
    }