/* enumeration which represents the state of a cell*/
#[derive(Clone)]
pub enum CellState {
	/* the cell is ALIVE during the CURRENT cycle */
	Alive,
	/* the cell is DEAD during the CURRENT cycle */
	Dead,

	/* the cell will be REVIVED in the NEXT cycle */
	WillBeRevived,

	/* the cell will be KILLED in the NEXT cycle */
	WillBeKilled
}

/* struct that stores a single variable representing the cell's state 
 * contains functions that tell and manipulate the cell's state */
#[derive(Clone)]
pub struct Cell {
	pub state: CellState
}

impl Cell {
	pub fn kill(&mut self) {
		self.state = CellState::Dead;
	}

	pub fn revive(&mut self) {
		self.state = CellState::Alive;
	}

	pub fn set_mark(&mut self) {
		match self.state {
			CellState::WillBeRevived => self.revive(),
			CellState::WillBeKilled => self.kill(),
			_ => ()
		}
	}

	pub fn mark_revive(&mut self) {
		self.state = CellState::WillBeRevived;
	}

	pub fn mark_death(&mut self) {
		self.state = CellState::WillBeKilled;
	}

	pub fn is_alive(&self) -> bool {
		match self.state {
			CellState::Alive => true,
			_ => false
		}
	}

	pub fn is_dead(&self) -> bool {
		!self.is_alive()
	}
}
