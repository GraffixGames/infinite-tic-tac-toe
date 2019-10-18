use crate::board::{Player, Board, BoardState};

#[derive(Clone, Debug)]
pub struct Game {
    pub(crate) board: Board,
    current_player: Player,
}

impl Game {
    pub fn new(depth: u8) -> Self {
        Game {
            board: Board::new(depth),
            current_player: Player::X,
        }
    }

    pub fn play_turn(&mut self) {
        self.board.play_turn(self.current_player);
        self.current_player = self.current_player.swap();
        println!();
    }

    pub fn winner(&self) -> Option<Option<Player>> {
        match self.board.state() {
            BoardState::Complete(winner) => Some(winner),
            BoardState::Running => None,
        }
    } 
}

