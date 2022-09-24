use crate::piece::PieceType;

use rand::prelude::*;

use enumset::EnumSet;
use enumset::EnumSetIter;

pub struct BagRandomizer<R>
where	R: Rng {
	set: EnumSet<PieceType>,
	rng: R,
}

impl<R> BagRandomizer<R>
where	R: Rng {
	pub fn new(rng: R) -> BagRandomizer<R> {
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