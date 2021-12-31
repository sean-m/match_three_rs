
pub enum Facet {
	None,
	Red,
	Grean,
	Blue,
	Square,
	Diamond,
	Circle,
}

impl Clone for Facet {
	// add code here
	fn clone(&self) -> Self { 
		let temp = *self;
		temp
	}
}

impl Copy for Facet { }

pub struct Piece {
	facets: (Facet, Facet, Facet),
}

impl Clone for Piece {
	fn clone(&self) -> Self { Piece { facets: self.facets.clone() } }
}

pub struct GameBoard {
	pub width: u8,
	pub height: u8,
	pub pieces: Vec<Piece>,
}

impl GameBoard {
	pub fn new (width: u8, height: u8) -> GameBoard {
		GameBoard {
			width,
			height,
			pieces: vec![Piece{ facets: (Facet::None, Facet::None, Facet::None)}; (width * height).into()]
		}
	}

	pub fn get_size (&self) -> usize {
		self.pieces.len()
	}
}

mod tests{
	use crate::board::GameBoard;
	
	#[test]
	fn board_size_correct() {
		assert_eq!(GameBoard::new(8,8).get_size(), 64);
	}
}