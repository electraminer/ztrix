
use crate::serialize::FromChars;
use std::ops::Index;
use std::ops::IndexMut;
use crate::game::PieceType;

use crate::game::BagRandomizer;
use crate::replay::Info;

use std::fmt;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Queue {
	pub queue: [PieceType; 4],
	pub rando: BagRandomizer,
}

impl Queue {
	pub fn new(mut rando: BagRandomizer,
			info: &mut Info) -> Queue {
		Queue{
			queue: [(); 4].map(|_| rando.next(info)),
			rando: rando,
		}
	}

	pub fn get_rando(&self) -> &BagRandomizer {
		&self.rando
	}

	pub fn get(&self, idx: usize) -> PieceType {
		self.queue[idx]
	}

	pub fn next(&mut self, info: &mut Info) -> PieceType {
		let next = self.queue[0];
		for i in 0..4-1 {
			self.queue[i] = self.queue[i + 1];
		}
		self.queue[4-1] = self.rando.next(info);
		next
	}
}

impl Index<usize> for Queue {
	type Output = PieceType;
	fn index(&self, index: usize) -> &Self::Output {
		&self.queue[index]
	}
}

impl IndexMut<usize> for Queue {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.queue[index]
	}
}

impl fmt::Display for Queue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for piece in self.queue.iter() {
			write!(f, "{}", piece)?;
		}
		write!(f, "{}", self.rando)
	}
}

impl FromChars for Queue {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut queue = [PieceType::I; 4];
		for piece in queue.iter_mut() {
			*piece = PieceType::from_chars(chars)?
		}
		Ok(Queue {
			queue: queue,
			rando: BagRandomizer::from_chars(chars)?,
		})
	}
}