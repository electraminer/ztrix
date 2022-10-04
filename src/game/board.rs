
use crate::serialize::FromChars;
use crate::serialize;
use crate::game::Mino;
use crate::position::Position;

use std::ops::IndexMut;
use std::ops::Index;

use std::fmt;

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

	pub fn clear_lines(&mut self) {
		for y in (0..26).rev() {
			if self.matrix[y].iter().all(|m|
					matches!(m, Some(_))) {
				for i in y..26-1 {
					self.matrix[i] = self.matrix[i+1];
				}
				self.matrix[26-1] = [None; 10];
			}
		}
	}

	pub fn clear_lines_zone(&mut self) {
		for y in 0..26 {
			if self.matrix[y].iter().all(|m|
					matches!(m, Some(_))) {
				for i in (0..y).rev() {
					self.matrix[i+1] = self.matrix[i];
				}
				self.matrix[0] = [Some(Mino::Gray); 10];
			}
		}
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

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in self.matrix.iter() {
			if row.iter().all(|m| matches!(m, None)) {
				write!(f, "E")?; // empty
			} else if row.iter().all(|m|
				matches!(m, Some(Mino::Gray))) {
				write!(f, "F")?; // full
			} else if row.iter().all(|m|
				matches!(m, None | Some(Mino::Gray))) {
				write!(f, "G")?; // grayscale
				let mut bin = 0;
				for mino in row.iter() {
					bin *= 2;
					if let Some(_) = mino {
						bin += 1
					}
				}
				serialize::write_b64_fixed(f, bin, 2)?;
			} else {
				write!(f, "C")?; // color
				for mino in row.iter() {
					match mino {
						None => write!(f, "_")?,
						Some(mino) => write!(f, "{}", mino)?,
					}
				}
			}
		}
		Ok(())
	}
}

impl FromChars for Board {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		let mut matrix = [[None; 10]; 26];
		for row in matrix.iter_mut() {
			match chars.next().ok_or(())? {
				'E' => *row = [None; 10],
				'F' => *row = [Some(Mino::Gray); 10],
				'G' => {
					let mut bin = serialize::read_b64_fixed(
						&mut chars, 2)?;
					for mino in row.iter_mut().rev() {
						*mino = match bin % 2 {
							0 => None,
							1 => Some(Mino::Gray),
							_ => return Err(()),
						};
						bin /= 2;
					}
				}
				'C' => {
					for mino in row.iter_mut() {
						*mino = match chars.peek().ok_or(())? {
							'_' => {
								chars.next();
								None
							},
							_ => Some(Mino::from_chars(
								&mut chars)?),
						}
					}
				}
				_ => return Err(()),
			}
		}
		Ok(Board {
			matrix: matrix,
		})
	}
}