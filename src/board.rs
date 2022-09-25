use crate::mino::Mino;
use crate::position::Position;

use std::ops::IndexMut;
use std::ops::Index;

pub struct Board<const W: usize = 10, const H: usize = 26> {
	matrix: [[Option<Mino>; W]; H]
}

impl<const W: usize, const H: usize> Board<W, H> {
	pub fn new() -> Board<W, H> {
		Board{
			matrix: [[None; W]; H]
		}
	}

	pub fn clear_lines(&mut self) {
		for y in 0..H {
			if self.matrix[y].iter().all(|m|
					matches!(m, Some(_))) {
				for i in (0..y).rev() {
					self.matrix[i+1] = self.matrix[i];
				}
				self.matrix[0] = [Some(Mino::Gray); W];
			}
		}
	}
}

impl<const W: usize, const H: usize> Index<Position> for Board<W, H> {
	type Output = Option<Mino>;
	fn index(&self, pos: Position) -> &Option<Mino> {
		if pos.x as usize >= W || pos.y as usize >= H {
			return &Some(Mino::Gray)
		}
		&self.matrix[pos.y as usize][pos.x as usize]
	}
}

impl<const W: usize, const H: usize> IndexMut<Position> for Board<W, H> {
	fn index_mut(&mut self, pos: Position) -> &mut Option<Mino> {
		&mut self.matrix[pos.y as usize][pos.x as usize]
	}
}