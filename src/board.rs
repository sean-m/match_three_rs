
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Facet {
	None,
	Red,
	Grean,
	Blue,
	Square,
	Diamond,
	Circle,
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
	facet1: Facet,
	facet2: Facet,
	facet3: Facet,
}

impl Piece {
	pub fn empty () -> Piece {
		Piece {
			facet1: Facet::None,
			facet2: Facet::None,
			facet3: Facet::None,
		}
	}
}


pub struct GameBoard {
	pub width: usize,
	pub height: usize,
	pub pieces: Vec<Piece>,
}

impl GameBoard {
	pub fn new (width: usize, height: usize) -> GameBoard {
		GameBoard {
			width: width.into(),
			height: height.into(),
			pieces: vec![Piece::empty(); (width * height).into()]
		}
	}

	pub fn get_size (&self) -> usize {
		self.pieces.len()
	}

	pub fn get_used_spaces(&self) -> usize {
		self.pieces.iter().filter(|&x| *x != Piece::empty()).count()
	}

	pub fn fill_board(&self) {
		while self.get_used_spaces() == 0 {
			for column in 0.. self.width-1 {
				self.drop_new_piece(column);
			}
		}
	}

	fn drop_new_piece(&self, column: usize) {
		// Place at top
		if self.spot_empty(0, column) {

		}

		// Fall down
		for row in 1..self.height-1 {
			if self.spot_empty(row, column) {

			}
		}
	}

	fn spot_empty(&self, x: usize, y: usize) -> bool {

		true
	}
}


mod tests{
	use crate::board::GameBoard;

	#[test]
	fn board_size_correct() {
		assert_eq!(GameBoard::new(8,8).get_size(), 64);
	}

	#[test]
	fn empty_board_used_spaces_eq_zero() {
		let board = GameBoard::new(8,8);
		assert_eq!(board.get_used_spaces(), 0);
	}

	#[test]
	fn filled_board_is_used_up() {
		let board = GameBoard::new(8,8);
		board.fill_board();
		assert_eq!(board.get_used_spaces(), 64);
	}
}