
use crate::game::PieceType;

use crate::game::BagRandomizer;
use crate::replay::Info;

#[derive(Clone)]
pub struct Queue<const L: usize = 4> {
	queue: [PieceType; L],
	rando: BagRandomizer,
}

impl<const L: usize> Queue<L> {
	pub fn new(mut rando: BagRandomizer,
			info: &mut Info) -> Queue<L> {
		Queue{
			queue: [(); L].map(|_| rando.next(info)),
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
		for i in 0..L-1 {
			self.queue[i] = self.queue[i + 1];
		}
		self.queue[L-1] = self.rando.next(info);
		next
	}
}