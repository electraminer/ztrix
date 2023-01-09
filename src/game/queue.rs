
use std::collections::VecDeque;
use rand::RngCore;

use crate::serialize::SerializeUrlSafe;
use std::ops::Index;
use std::ops::IndexMut;
use crate::game::PieceType;

use crate::game::BagRandomizer;

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

	pub fn get_num_revealed(&self) -> usize {
		self.pieces.len()
	}

	pub fn get(&self, idx: usize) -> Option<PieceType> {
		self.pieces.get(idx).cloned()
	}

	pub fn next(&mut self) -> Result<PieceType, usize> {
		self.pieces.pop_front().ok_or(rand::thread_rng().next_u64() as usize)
	}

	pub fn reveal(&mut self, seed: usize) {
		self.pieces.push_back(self.rando.next(seed));
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