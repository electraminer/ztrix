use crate::piece::PieceType;

use rand::prelude::*;

use enumset::EnumSet;
use enumset::EnumSetIter;

pub struct BagRandomizer {
	set: EnumSet<PieceType>,
	rng: ThreadRng,
}

impl BagRandomizer {
	pub fn new(rng: ThreadRng) -> BagRandomizer {
		BagRandomizer{
			set: EnumSet::all(),
			rng: rng,
		}
	}

	pub fn options(&self) -> EnumSetIter<PieceType> {
		self.set.iter()
	}

	pub fn next(&mut self) -> PieceType {
		let next = self.options().choose(&mut self.rng)
			.expect("should always be at least one option");
		self.set -= next;
		if self.set.is_empty() {
			self.set = EnumSet::all();
		}
		next
	}
}