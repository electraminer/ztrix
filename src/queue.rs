use crate::piece::PieceType;

use crate::randomizer::BagRandomizer;

pub struct Queue<const L: usize = 4> {
	queue: [PieceType; L],
	rando: BagRandomizer,
}

impl<const L: usize> Queue<L> {
	pub fn new(mut rando: BagRandomizer) -> Queue<L> {
		Queue{
			queue: [(); L].map(|_| rando.next()),
			rando: rando,
		}
	}

	pub fn get(&self, idx: usize) -> PieceType {
		self.queue[idx]
	}

	pub fn next(&mut self) -> PieceType {
		let next = self.queue[0];
		for i in 0..L-1 {
			self.queue[i] = self.queue[i + 1];
		}
		self.queue[L-1] = self.rando.next();
		next
	}
}