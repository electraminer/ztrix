use crate::position::Vector;

extern crate enumset;
use enumset::EnumSetType;

#[derive(EnumSetType)]
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
			PieceType::S => [Vector::new(-1, 0), Vector::new(0, 0),
					Vector::new(0, 1), Vector::new(1, 1)],
			PieceType::Z => [Vector::new(1, 0), Vector::new(0, 0),
					Vector::new(0, 1), Vector::new(-1, 1)],
			PieceType::J => [Vector::new(-1, 0), Vector::new(0, 0),
					Vector::new(1, 0), Vector::new(-1, 1)],
			PieceType::T => [Vector::new(-1, 0), Vector::new(0, 0),
					Vector::new(1, 0), Vector::new(0, 1)],
			PieceType::L => [Vector::new(-1, 0), Vector::new(0, 0),
					Vector::new(1, 0), Vector::new(1, 1)],
			PieceType::O => [Vector::new(0, 0), Vector::new(1, 0),
					Vector::new(0, 1), Vector::new(1, 1)],
			PieceType::I => [Vector::new(-1, 0), Vector::new(0, 0),
					Vector::new(1, 0), Vector::new(2, 0)],
		}
	}
}