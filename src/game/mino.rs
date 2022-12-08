use crate::serialize::SerializeUrlSafe;
use crate::game::PieceType;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}

impl SerializeUrlSafe for Mino {
	fn serialize(&self) -> String {
		match self {
			Mino::Piece(piece) => piece.serialize(),
			Mino::Gray => "G".to_owned(),
		}
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(if input.next_if('G')? {
			Mino::Gray
		} else {
			Mino::Piece(PieceType::deserialize(input)?)
		})
	}
}