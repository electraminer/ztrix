
use crate::serialize::FromChars;
use crate::game::PieceType;
use crate::replay::Info;

use enumset::EnumSet;
use enumset::EnumSetIter;

use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BagRandomizer {
	pub set: EnumSet<PieceType>,
}

impl BagRandomizer {
	pub fn new() -> BagRandomizer {
		BagRandomizer{
			set: EnumSet::all(),
		}
	}

	pub fn options(&self) -> EnumSetIter<PieceType> {
		self.set.iter()
	}

	pub fn next(&mut self, info: &mut Info) -> PieceType {
		let options = self.options().collect();
		let next = *info.choice(&options);
		self.set -= next;
		if self.set.is_empty() {
			self.set = EnumSet::all();
		}
		next
	}
}

impl fmt::Display for BagRandomizer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for piece in self.set.iter() {
			write!(f, "{}", piece)?;
		}
		write!(f, ".")
	}
}

impl FromChars for BagRandomizer {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		let mut set = EnumSet::empty();
		while *chars.peek().ok_or(())? != '.' {
			let piece = PieceType::from_chars(&mut chars)?;
			set.insert(piece);
		}
		Ok(BagRandomizer {
			set: set,
		})
	}
}