use std;
use std::fmt;

use std::rc::Rc;


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
	pub Name: String
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

	// TODO put this somewhere else, the board doesn't need to know how to fill itself. That's a function of the game.
	// pub fn fill_board(&mut self) {
	// 	while self.get_used_spaces() == 0 {
	// 		for column in 0.. self.width-1 {
	// 			self.drop_new_piece(column);
	// 		}
	// 	}
	// }

	// TODO put this somewhere else, the board doesn't need to know how to fill itself. That's a function of the game.
	// fn drop_new_piece(&mut self, column: usize) {
	// 	// Place at top
	// 	if self.spot_empty(0, column) {
	// 		let token = Piece{ 
	// 			facet1: String::from("G"),
	// 			facet2: String::from(""),
	// 			facet3: String::from(""),
	// 		};
	// 		//TODO Make this select random facets
	// 		//TODO Make randomness depend on gmae rules
	// 		self.place_token(token, 0, column);
	// 	}
	// }

	fn spot_empty(&self, row: usize, column: usize) -> bool {
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

		let before = board.spot_empty(1,1);
		board.place_token(token, 1, 1).unwrap();

		let after = board.spot_empty(1,1);
		
		assert_ne!(before, after)
	}

	/*
	#[test]
	fn filled_board_is_used_up() {
		let board = GameBoard::new(8,8);
		board.fill_board();
		assert_eq!(board.get_used_spaces(), 64);
	}
	*/

	#[test]
	fn wont_place_tokens_out_of_bounds() {
		let mut board = GameBoard::new(8,8);
		let token = Piece::new("G","","");
		let token2 = Piece::new("G","","");

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