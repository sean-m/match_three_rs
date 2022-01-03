use std;
use std::fmt;


#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Facet {
	None,
	Red,
	Green,
	Blue,
	Square,
	Diamond,
	Circle,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
	facet1: String,
	facet2: String,
	facet3: String,
}

impl Piece {
	pub fn empty () -> Piece {
		Piece {
			facet1: String::from(""),
			facet2: String::from(""),
			facet3: String::from(""),
		}
	}

	pub fn new (
		facet_val1: &str,
		facet_val2: &str,
		facet_val3: &str,
		) -> Piece {
		Piece {
			facet1: String::from(facet_val1),
			facet2: String::from(facet_val2),
			facet3: String::from(facet_val3),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Augmentation {
	pub name: String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spot {
	pub token: Piece,
	pub augmentations: Vec<Augmentation>
}

impl Spot {
	fn empty() -> Self {
		Spot {
			token: Piece::empty(),
			augmentations: vec![]
		}
	}
}


#[derive(Debug, Clone)]
pub struct BoardError {
	pub message: String,
}

impl fmt::Display for BoardError {
	fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

impl std::error::Error for BoardError {
	fn description(&self) -> &str {
		&self.message
	}
}


#[derive(Debug, Clone, PartialEq)]
pub struct GameBoard {
	pub width: usize,
	pub height: usize,
	pub spots: Vec<Spot>,
}

impl GameBoard {
	pub fn new (width: usize, height: usize) -> GameBoard {
		GameBoard {
			width: width.into(),
			height: height.into(),
			spots: vec![Spot::empty(); (width * height).into()]
		}
	}

	pub fn get_size (&self) -> usize {
		self.spots.len()
	}

	pub fn get_used_spaces(&self) -> usize {
		self.spots.iter().filter(|&x| *x != Spot::empty()).count()
	}

	fn is_spot_empty(&self, row: usize, column: usize) -> bool {
		let offset = self.width * (row - 1);
		let offset = offset + column - 1;
		self.spots[offset].token == Piece::empty()
	}

	pub fn place_token(&mut self, token: Piece, row: usize, column: usize) 
		-> Result<String, BoardError> {
		if 0 >= row || 0 >= column 
		|| row > self.height || column > self.width {
			return Err(BoardError { message: "Out of bounds".to_string() });
		}

		let offset = self.width * (row - 1);
		let offset = offset + column - 1;
		self.spots[offset].token = token;
		
		Ok("Success".to_string())
	}
}


mod tests{
	use crate::board::*;

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
	fn placing_token_uses_spaces() {
		let mut board = GameBoard::new(8,8);
		let token = Piece::new("G","","");

		let before = board.get_used_spaces();
		board.place_token(token, 1, 1).unwrap();
		
		assert_ne!(before, board.get_used_spaces());
	}

	#[test]
	fn placed_token_is_no_longer_empty() {
		let mut board = GameBoard::new(8,8);
		let token = Piece::new("G","","");

		let before = board.is_spot_empty(1,1);
		board.place_token(token, 1, 1).unwrap();

		let after = board.is_spot_empty(1,1);
		
		assert_ne!(before, after)
	}

	#[test]
	fn peices_with_the_same_facets_are_the_same () {
		let token1 = Piece::new("Green","Square","");
		let token2 = Piece::new("Green","Square","");
		let token3 = Piece::new("Red","Square","");
		let token4 = Piece::new("Green","Circle","");

		assert_eq!(token1, token2);
		assert_ne!(token1, token3);
		assert_ne!(token1, token4);
		assert_ne!(token3, token4);
	}

	#[test]
	fn wont_place_tokens_out_of_bounds() {
		let mut board = GameBoard::new(8,8);
		let token = Piece::new("Green","","");
		let token2 = Piece::new("Green","","");

		match board.place_token(token, 9, 1) {
			Ok(_) => assert!(false, "Column 9 should be out of bounds on 8x8 board."),
			Err(_) => assert!(true),
		}

		match board.place_token(token2, 1, 1) {
			Ok(_) => assert!(true),
			Err(_) => assert!(false, "Location 1,1 should be in bounds on 8x8 board."),
		}
	}
}