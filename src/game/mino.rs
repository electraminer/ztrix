use crate::serialize::FromChars;
use crate::game::PieceType;

use std::fmt;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}

impl fmt::Display for Mino {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Mino::Piece(piece) => write!(f, "{}", piece),
			Mino::Gray => write!(f, "G"),
		}
	}
}

impl FromChars for Mino {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		Ok(match chars.peek().ok_or(())? {
			'G' => {
				chars.next();
				Mino::Gray
			},
			_ => Mino::Piece(PieceType::from_chars(&mut chars)?),
		})
	}
}