use std::io::{self, Write};
const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn swap(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl<'a> Into<&'a str> for Player {
    fn into(self) -> &'a str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Player(Option<Player>),
    Complete(Option<Player>),
    InProgress(Option<Box<Board>>),
}

impl Tile {
    pub fn is_occupied(&self) -> bool {
        match *self {
            Tile::Player(Some(_)) => true,
            Tile::Complete(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    tiles: Vec<Box<Tile>>,
    depth: u8,
}

pub enum BoardState {
    Complete(Option<Player>),
    Running,
}

impl Board {
    pub fn new(depth: u8) -> Self {
        let tiles = if depth == 0 {
            vec![Box::new(Tile::Player(None)); BOARD_SIZE * BOARD_SIZE]
        } else {
            vec![Box::new(Tile::InProgress(None)); BOARD_SIZE * BOARD_SIZE]
        };

        Board { tiles, depth }
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
                let tile = match *self.tiles[Self::board_pos(row, col)] {
                    Tile::Player(Some(p)) => p.into(),
                    Tile::Player(None) => "-",
                    Tile::Complete(Some(p)) => p.into(),
                    Tile::Complete(None) => "*",
                    _ => "?",
                };
                print!("{}", tile);
            }
            println!("");
        }
    }

    fn board_pos(row: usize, col: usize) -> usize {
        row * BOARD_SIZE + col
    }

    pub fn state(&self) -> BoardState {
        let mut combos = Vec::new();
        combos.extend({
            let mut row_combos = Vec::new();
            for row in 0..BOARD_SIZE {
                let mut combo = Vec::with_capacity(BOARD_SIZE);
                for col in 0..BOARD_SIZE {
                    combo.push(&self.tiles[Self::board_pos(row, col)]);
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
                    combo.push(&self.tiles[Self::board_pos(row, col)]);
                }
                column_combos.push(combo);
            }
            column_combos
        });
        combos.extend({
            let mut tl_br = Vec::with_capacity(BOARD_SIZE);
            let mut tr_bl = Vec::with_capacity(BOARD_SIZE);
            for coord in 0..BOARD_SIZE {
                tl_br.push(&self.tiles[Self::board_pos(coord, coord)]);
                tr_bl.push(&self.tiles[Self::board_pos(coord, BOARD_SIZE - 1 - coord)]);
            }
            vec![tl_br, tr_bl]
        });

        for combo in combos.clone() {
            let player = match **combo[0] {
                Tile::Player(p) => p,
                Tile::Complete(p) => p,
                _ => None,
            };

            match player {
                Some(p) => {
                    if combo.iter().all(|tile| {
                        ***tile == Tile::Complete(Some(p)) || ***tile == Tile::Player(Some(p))
                    }) {
                        return BoardState::Complete(Some(p));
                    }
                }
                _ => (),
            }
        }

        if combos
            .iter()
            .all(|row| row.iter().all(|tile| tile.is_occupied()))
        {
            BoardState::Complete(None)
        } else {
            BoardState::Running
        }
    }

    // row and col are 1 indexed
    // returns the validity of the move
    fn make_move(&mut self, player: Player, row: usize, col: usize) -> bool {
        let row = row - 1;
        let col = col - 1;
        match *self.tiles[Self::board_pos(row, col)].clone() {
            Tile::Player(ref p) => match p {
                Some(_) => false,
                None => {
                    self.tiles[Self::board_pos(row, col)] = Box::new(Tile::Player(Some(player)));
                    true
                }
            },
            Tile::Complete(_) => false,
            Tile::InProgress(ref mut board) => match board {
                Some(ref mut board) => {
                    self.tiles[Self::board_pos(row, col)] = match board.play_turn(player) {
                        BoardState::Complete(w) => Box::new(Tile::Complete(w)),
                        BoardState::Running => {
                            Box::new(Tile::InProgress(Some(Box::new(*board.clone()))))
                        }
                    };
                    true
                }
                None => {
                    self.tiles[Self::board_pos(row, col)] =
                        Box::new(Tile::InProgress(Some(Box::new(Board::new(self.depth - 1)))));
                    self.make_move(player, row + 1, col + 1)
                }
            },
        }
    }

    pub fn play_turn(&mut self, player: Player) -> BoardState {
        self.draw();
        loop {
            let (row, col) = prompt(player);
            if self.make_move(player, row, col) {
                break self.state();
            }
            println!("That spot is occupied");
        }
    }
}

fn prompt(player: Player) -> (usize, usize) {
    let player: &str = player.into();

    loop {
        print!(
            "{}, please type the location of your move (row, column): ",
            player
        );
        io::stdout().flush().expect("could not flush stdout");
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => std::process::exit(0), // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading line! {}", e);
                std::process::exit(1);
            }
        }
        let (row, col) = {
            let parsed_input: Vec<char> = buffer.chars().filter(|c| c.is_numeric()).collect();

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
                row, col, BOARD_SIZE,
            )
        }
    }
}
