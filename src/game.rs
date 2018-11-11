use super::{Tile, Board, BoardState, Winner};

#[derive(Clone, Debug)]
pub struct Game {
    pub(crate) board: Board,
    current_player: Tile,
}

impl Game {
    pub fn new(depth: u32) -> Self {
        Game {
            board: Board::new(depth),
            current_player: Tile::X,
        }
    }

	pub fn play_turn(&mut self) {
		self.board.play_turn(self.current_player);
		self.current_player = self.current_player.swap();
	}

	pub fn winner(&self) -> Option<Winner> {
		match self.board.state() {
			BoardState::Complete(winner) => Some(winner),
			BoardState::Running => None,
		}
	} 
}