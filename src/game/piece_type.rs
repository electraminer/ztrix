use crate::position::Rotation;
use crate::position::Vector;

extern crate enumset;
use enumset::EnumSetType;

#[derive(Debug, EnumSetType, Hash)]
pub enum PieceType {
	S,
	T,
	Z,
	L,
	O,
	J,
	I,
}

impl PieceType {
	pub fn get_mino_vecs(self) -> [Vector; 4] {
		match self {
			PieceType::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
			PieceType::Z => [(1, 0), (0, 0), (0, 1), (-1, 1)],
			PieceType::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
			PieceType::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
			PieceType::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
			PieceType::O => [(0, 0), (1, 0), (0, -1), (1, -1)],
			PieceType::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
		}.map(|(x, y)| Vector::new(x, y))
	}

	pub fn get_szjtl_offsets(rot: Rotation) -> [Vector; 5] {
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

	pub fn get_oi_offsets(rot: Rotation) -> [Vector; 5] {
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

	pub fn get_offsets(self, rot: Rotation) -> [Vector; 5] {
		match self {
			PieceType::O | PieceType::I =>
					PieceType::get_oi_offsets(rot),
			_ => PieceType::get_szjtl_offsets(rot),
		}
	}

	pub fn get_kicks(self, start: Rotation, rot: Rotation)
			-> [Vector; 5] {
		let start_offsets = self.get_offsets(start);
		let end_offsets = self.get_offsets(start + rot);
		start_offsets.zip(end_offsets).map(|(s, e)| s - e)
	}
}