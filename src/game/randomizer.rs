
use crate::serialize::SerializeUrlSafe;
use crate::game::PieceType;
use crate::replay::Info;

use enumset::EnumSet;
use enumset::EnumSetIter;

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

impl SerializeUrlSafe for BagRandomizer {
	fn serialize(&self) -> String {
		let vec: Vec<PieceType> = self.set.iter().collect();
		vec.serialize()
	}
	
	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		let vec: Vec<PieceType> = Vec::deserialize(input)?;
		Ok(BagRandomizer {
			set: EnumSet::from_iter(vec.into_iter())
		})
	}
}