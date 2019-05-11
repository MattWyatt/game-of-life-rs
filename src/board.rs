use crate::cell::{CellState, Cell};

/* structure that represents the game board
 * contains only methods for getting data about cells in the board */
 #[derive(Clone)]
pub struct Board {
	width: u32,
	height: u32,
	board: Vec<Cell>
}

impl Board {
	/* instantiates a new board
	 * automatically fills the board's array with cells */
	pub fn new(width: u32, height: u32) -> Board {
		let size = width * height;

		Board {
			width,
			height,
			board: vec![Cell {state: CellState::Dead}; size as usize]
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	/* function that lets the 1 dimensional vector to act as a 2 dimensional array
	 * returns a reference to the cell at the specified coordinates
	 * if the supplied coordinates are invalid, the option holds nothing */
	fn _at(&self, row: i32, col: i32) -> Option<&Cell> {
		//let index = ((y - 1) * self.width) + x;
		let index = (col * self.width as i32) + (row + 1);
		
		self.board.get(index as usize)
	}

	/* returns a reference to the cell at the specified index
	 * this function will panic if you attempts to access an invalid index */
	pub fn at(&self, row: i32, col: i32) -> &Cell {
		match self._at(row, col) {
			Some(cell) => cell,
			_ => panic!("attempted to access invalid index of board!")
		}
	}

	/* returns a mutable reference to the specified cell
	 * just like Board.at, this function will panic if you attempt to access an invalid index */
	pub fn get(&mut self, row: i32, col: i32) -> &mut Cell {
		let index = (col * self.width as i32) + (row + 1);

		match self.board.get_mut(index as usize) {
			Some(cell) => cell,
			_ => panic!("attempted to access invalid mutable index of board!")
		}
	}

	/* returns the count of living neighbors a given cell has */
	pub fn living_neighbors(&self, row: i32, col: i32) -> u32 {
		let mut total = 0;

		let neighbors = [
			(row, col + 1),     /* north */
			(row + 1, col),     /* east */
			(row, col - 1),     /* south */
			(row - 1, col),     /* west */
			(row + 1, col + 1), /* north east */
			(row + 1, col - 1), /* south east */
			(row - 1, col - 1), /* south west */
			(row - 1, col + 1)  /* north west */
		];

		/* iterate through all the neighbor positions */
		for n in neighbors.iter() {
			/* use _at and handle the _ case for the match arm
			 * this way we don't have to check if the coords are within bounds */
			match self._at(n.0, n.1) {
				Some(cell) => {
					/* increment the counter if the cell is alive */
					if cell.is_alive() {
						total = total + 1;
					}
				},
				_ => ()
			}
		}

		total
	}
}