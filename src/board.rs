use std::io::{self, Write};
const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plr {
    X,
    O,
}

impl Plr {
    pub fn swap(self) -> Self {
        match self {
            Plr::X => Plr::O,
            Plr::O => Plr::X,
        }
    }
}

impl<'a> Into<&'a str> for Plr {
    fn into(self) -> &'a str {
        match self {
            Plr::X => "X",
            Plr::O => "O",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Plr(Option<Plr>),
    Board(Box<Board>),
}

impl Tile {
    pub fn is_player(&self) -> bool {
        match *self {
            Tile::Plr(Some(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardState {
    Complete(Option<Plr>),
    Running,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    tiles: Vec<Vec<Box<Tile>>>,
}

impl Board {
    pub fn new(depth: u32) -> Self {
        let tiles = if depth == 0 {
            vec![vec![Box::new(Tile::Plr(None)); BOARD_SIZE]; BOARD_SIZE]
        } else {
            vec![vec![Box::new(Tile::Board(Box::new(Board::new(depth - 1)))); BOARD_SIZE]; BOARD_SIZE]
        };

        Board {
            tiles,
        }
    }

    fn draw(&self) {
        print!("  ");
        for column_marker in 1..=BOARD_SIZE {
            print!("{} ", column_marker);
        }
        println!("");
        for row in 0..BOARD_SIZE {
            print!("{}", row + 1);
            for col in 0..BOARD_SIZE {
                print!(" ");
                let tile = if let Tile::Plr(Some(p)) = *self.tiles[row][col] {
                     p.into()
                } else if let Tile::Plr(None) = *self.tiles[row][col] {
                    "-"
                } else {
                    "?"
                };
                print!("{}", tile);
            }
            println!("");
        }
    }

    pub fn state(&self) -> BoardState {
        let mut combos = Vec::new();
        combos.extend({
            let mut row_combos = Vec::new();
            for row in 0..BOARD_SIZE {
                let mut combo = Vec::with_capacity(BOARD_SIZE);
                for col in 0..BOARD_SIZE {
                    combo.push(&self.tiles[row][col]);
                }
                row_combos.push(combo);
            }
            row_combos
        });
        combos.extend({
            let mut column_combos = Vec::new();
            for col in 0..BOARD_SIZE {
                let mut combo = Vec::with_capacity(BOARD_SIZE);
                for row in 0..BOARD_SIZE {
                    combo.push(&self.tiles[row][col]);
                }
                column_combos.push(combo);
            }
            column_combos
        });
        combos.extend({
            let mut tl_br = Vec::with_capacity(BOARD_SIZE);
            let mut tr_bl = Vec::with_capacity(BOARD_SIZE);
            for coord in 0..BOARD_SIZE {
                tl_br.push(&self.tiles[coord][coord]);
                tr_bl.push(&self.tiles[coord][BOARD_SIZE - 1 - coord]);
            }
            vec![tl_br, tr_bl]
        });

        for combo in combos.clone() {
            if let Tile::Plr(Some(p)) = **combo[0] {
                if combo.iter().all(|t| ***t == Tile::Plr(Some(p))) {
                    return BoardState::Complete(Some(p))
                }
            }
        };

        if combos.iter().all(|row| row.iter().all(|tile| tile.is_player())) {
            BoardState::Complete(None)
        } else {
            BoardState::Running
        }
    }

    fn make_move(&mut self, player: Plr, row: usize, col: usize) -> Result<(), ()> {
        match *self.tiles[row - 1][col - 1].clone() {
            Tile::Plr(ref p) => match p {
                Some(_) => Err(()),
                None => {
                    self.tiles[row - 1][col - 1] = Box::new(Tile::Plr(Some(player)));
                    Ok(())
                },
            },
            Tile::Board(ref mut b) => {
                self.tiles[row - 1][col - 1] = match b.play_turn(player) {
                    BoardState::Complete(w) => Box::new(Tile::Plr(w)),
                    BoardState::Running => Box::new(Tile::Board(Box::new(*b.clone()))),
                };
                Ok(())
            },
        }
    }

    pub fn play_turn(&mut self, player: Plr) -> BoardState {
        self.draw();
        loop {
            let (row, col) = prompt(player);
            if self.make_move(player, row, col).is_ok() {
                break self.state();
            }
            println!("That spot is occupied");
        }
    }
}

fn prompt(player: Plr) -> (usize, usize) {
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