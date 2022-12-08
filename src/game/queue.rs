
use std::collections::VecDeque;
use crate::serialize::SerializeUrlSafe;
use std::ops::Index;
use std::ops::IndexMut;
use crate::game::PieceType;

use crate::game::BagRandomizer;
use crate::replay::Info;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Queue {
	pub length: usize,
	pub pieces: VecDeque<PieceType>,
	pub rando: BagRandomizer,
}

impl Queue {
	pub fn new(rando: BagRandomizer, length: usize) -> Queue {
		Queue{
			length: length,
			pieces: VecDeque::new(),
			rando: rando,
		}
	}

	pub fn fill(&self) -> usize {
		self.pieces.len()
	}

	pub fn get(&self, idx: usize) -> PieceType {
		self.pieces[idx]
	}

	pub fn update(&mut self, info: &mut Info) {
		while self.fill() < self.length {
			self.pieces.push_back(self.rando.next(info));
		}
	}

	pub fn next(&mut self, info: &mut Info) -> PieceType {
		let next = self.pieces.pop_front().unwrap_or_else(
			|| self.rando.next(info));
		self.update(info);
		next
	}
}

impl Index<usize> for Queue {
	type Output = PieceType;
	fn index(&self, index: usize) -> &Self::Output {
		&self.pieces[index]
	}
}

impl IndexMut<usize> for Queue {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.pieces[index]
	}
}

impl SerializeUrlSafe for Queue {
	fn serialize(&self) -> String {
		format! { "Q{}{}{}",
			self.length.serialize(),
			self.pieces.iter().cloned().collect::<Vec<PieceType>>().serialize(),
			self.rando.serialize(),
		}
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(if input.next_if('Q')? {
			Self {
				length: usize::deserialize(input)?,
				pieces: Vec::deserialize(input)?.iter().cloned().collect::<VecDeque<PieceType>>(),
				rando: BagRandomizer::deserialize(input)?,
			}
		} else {
			Self {
				length: 4,
				pieces: <[PieceType; 4]>::deserialize(input)?.iter().cloned().collect::<VecDeque<PieceType>>(),
				rando: BagRandomizer::deserialize(input)?,
			}
		})
	}
}