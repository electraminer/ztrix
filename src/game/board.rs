
use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;
use crate::game::Mino;
use crate::position::Position;

use std::ops::IndexMut;
use std::ops::Index;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Board {
	pub matrix: [[Option<Mino>; 10]; 26]
}

impl Board {
	pub fn new() -> Board {
		Board{
			matrix: [[None; 10]; 26]
		}
	}

	pub fn in_bounds(pos: Position) -> bool {
		((pos.x as usize) < 10) && ((pos.y as usize) < 26)
	}

	pub fn clear_lines(&mut self) -> usize {
		let mut cleared = 0;
		for y in (0..26).rev() {
			if self.matrix[y].iter().all(|m|
					matches!(m, Some(_))) {
				for i in y..26-1 {
					self.matrix[i] = self.matrix[i+1];
				}
				self.matrix[26-1] = [None; 10];
				cleared += 1;
			}
		}
		cleared
	}

	pub fn clear_lines_zone(&mut self) -> usize {
		let mut cleared = 0;
		for y in 0..26 {
			if self.matrix[y].iter().all(|m|
					matches!(m, Some(_))) {
				if !self.matrix[y].iter().all(|m|
					matches!(m, Some(Mino::Gray))) {
					cleared += 1;
				}
				for i in (0..y).rev() {
					self.matrix[i+1] = self.matrix[i];
				}
				self.matrix[0] = [Some(Mino::Gray); 10];
			}
		}
		cleared
	}
}

impl Index<Position> for Board {
	type Output = Option<Mino>;
	fn index(&self, pos: Position) -> &Option<Mino> {
		if pos.x as usize >= 10 || pos.y as usize >= 26 {
			return &Some(Mino::Gray)
		}
		&self.matrix[pos.y as usize][pos.x as usize]
	}
}

impl IndexMut<Position> for Board {
	fn index_mut(&mut self, pos: Position) -> &mut Option<Mino> {
		&mut self.matrix[pos.y as usize][pos.x as usize]
	}
}

impl SerializeUrlSafe for Board {
	fn serialize(&self) -> String {
		self.matrix.iter().map(|row|
			if row.iter().cloned().all(|m| m == None) {
				"E".to_owned()
			} else if row.iter().cloned().all(|m| m == Some(Mino::Gray)) {
				"F".to_owned()
			} else if row.iter().cloned().all(|m| m == None || m == Some(Mino::Gray)) {
				let mut row = row.clone();
				row.reverse();
				format! {"G{}", row.map(|m| m == Some(Mino::Gray)).serialize()}
			} else {
				format! {"C{}", row.serialize()}
			}
		).collect()
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		let mut matrix = [[None; 10]; 26];
		for row in matrix.iter_mut() {
			match input.next()? {
				'E' => *row = [None; 10],
				'F' => *row = [Some(Mino::Gray); 10],
				'G' => {
					*row = <[bool; 10]>::deserialize(input)?.map(|b| b.then_some(Mino::Gray));
					row.reverse();
				}
				'C' => *row = <[Option<Mino>; 10]>::deserialize(input)?,
				_ => return Err(DeserializeError::new("Row encoding types should be represented by E, F, G, or C.")),
			}
		}
		Ok(Board {
			matrix: matrix,
		})
	}
}