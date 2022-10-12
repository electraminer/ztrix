
use std::collections::VecDeque;
use crate::serialize::FromChars;
use std::ops::Index;
use std::ops::IndexMut;
use crate::game::PieceType;

use crate::game::BagRandomizer;
use crate::replay::Info;
use crate::serialize;
use std::fmt;

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

impl fmt::Display for Queue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Q")?;
		serialize::write_b64_var(f, self.length)?;
		for piece in self.pieces.iter() {
			write!(f, "{}", piece)?;
		}
		write!(f, ".{}", self.rando)
	}
}

impl FromChars for Queue {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		if *chars.peek().ok_or(())? == 'Q' {
			chars.next();
			let length = serialize::read_b64_var(&mut chars)?;
			let mut pieces = VecDeque::new();
			while *chars.peek().ok_or(())? != '.' {
				pieces.push_back(
					PieceType::from_chars(&mut chars)?);
			}
			chars.next();
			Ok(Queue {
				length: length,
				pieces: pieces,
				rando: BagRandomizer::from_chars(&mut chars)?,
			})
		} else {
			let mut pieces = VecDeque::new();
			for _ in 0..4 {
				pieces.push_back(
					PieceType::from_chars(&mut chars)?);
			}
			Ok(Queue {
				length: 4,
				pieces: pieces,
				rando: BagRandomizer::from_chars(&mut chars)?,
			})
		}
	}
}