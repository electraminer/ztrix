use crate::serialize::FromChars;
use crate::position::Rotation;
use crate::position::Vector;

extern crate enumset;
use enumset::EnumSetType;

use std::fmt;

#[derive(Debug, EnumSetType, Hash)]
pub enum PieceType {
	I,
	O,
	S,
	Z,
	J,
	L,
	T,
}

impl PieceType {
	pub fn get_mino_vecs(self) -> [Vector; 4] {
		match self {
			PieceType::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
			PieceType::O => [(0, 0), (1, 0), (0, -1), (1, -1)],
			PieceType::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
			PieceType::Z => [(1, 0), (0, 0), (0, 1), (-1, 1)],
			PieceType::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
			PieceType::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
			PieceType::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
		}.map(|(x, y)| Vector::new(x, y))
	}

	pub fn get_io_offsets(rot: Rotation) -> [Vector; 5] {
		match rot {
			Rotation::Zero =>
					[(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
			Rotation::Clockwise =>
					[(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
			Rotation::Flip =>
					[(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
			Rotation::Anticlockwise =>
					[(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
		}.map(|(x, y)| Vector::new(x, y))
	}

	pub fn get_szjlt_offsets(rot: Rotation) -> [Vector; 5] {
		match rot {
			Rotation::Zero =>
					[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
			Rotation::Clockwise =>
					[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
			Rotation::Flip =>
					[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
			Rotation::Anticlockwise =>
					[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
		}.map(|(x, y)| Vector::new(x, y))
	}

	pub fn get_offsets(self, rot: Rotation) -> [Vector; 5] {
		match self {
			PieceType::I | PieceType::O =>
					PieceType::get_io_offsets(rot),
			_ => PieceType::get_szjlt_offsets(rot),
		}
	}

	pub fn get_kicks(self, start: Rotation, rot: Rotation)
			-> [Vector; 5] {
		let start_offsets = self.get_offsets(start);
		let end_offsets = self.get_offsets(start + rot);
		start_offsets.zip(end_offsets).map(|(s, e)| s - e)
	}
}

impl fmt::Display for PieceType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let chr = match self {
			PieceType::I => 'I',
			PieceType::O => 'O',
			PieceType::S => 'S',
			PieceType::Z => 'Z',
			PieceType::J => 'J',
			PieceType::L => 'L',
			PieceType::T => 'T',
		};
		write!(f, "{}", chr)
	}
}

impl FromChars for PieceType {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		Ok(match chars.next().ok_or(())? {
			'I' | 'i' => PieceType::I,
			'O' | 'o' => PieceType::O,
			'S' | 's' => PieceType::S,
			'Z' | 'z' => PieceType::Z,
			'J' | 'j' => PieceType::J,
			'L' | 'l' => PieceType::L,
			'T' | 't' => PieceType::T,
			_ => return Err(())
		})
	}
}