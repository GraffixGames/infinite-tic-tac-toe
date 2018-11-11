use super::{Plr, Board, BoardState};

#[derive(Clone, Debug)]
pub struct Game {
    pub(crate) board: Board,
    current_player: Plr,
}

impl Game {
    pub fn new(depth: u32) -> Self {
        Game {
            board: Board::new(depth),
            current_player: Plr::X,
        }
    }

	pub fn play_turn(&mut self) {
		self.board.play_turn(self.current_player);
		self.current_player = self.current_player.swap();
		println!();
	}

	pub fn winner(&self) -> Option<Option<Plr>> {
		match self.board.state() {
			BoardState::Complete(winner) => Some(winner),
			BoardState::Running => None,
		}
	} 
}