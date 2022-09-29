use rand::prelude::IteratorRandom;
use crate::game::PieceType;
use crate::replay::Info;

use enumset::EnumSet;
use enumset::EnumSetIter;

#[derive(Clone)]
pub struct BagRandomizer {
	set: EnumSet<PieceType>,
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
		let next = self.options().choose(info)
			.expect("should always be at least one option");
		self.set -= next;
		if self.set.is_empty() {
			self.set = EnumSet::all();
		}
		next
	}
}