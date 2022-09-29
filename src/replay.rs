use rand::RngCore;
use rand::SeedableRng;
use crate::game::Action;

use crate::game::Game;

use rand::rngs::SmallRng;

struct Frame {
	actions: Vec<Action>,
}

impl Frame {
	pub fn new() -> Frame {
		Frame{
			actions: Vec::new(),
		}
	}
}

#[derive(Clone)]
pub struct Info {
	rng: SmallRng,
	revealed: bool,
}

impl Info {
	pub fn new() -> Info {
		Info {
			rng: SmallRng::from_entropy(),
			revealed: false,
		}
	}

	pub fn revealed(&self) -> bool {
		self.revealed
	}
}

impl RngCore for Info {
	fn next_u32(&mut self) -> u32 {
		self.revealed = true;
		self.rng.next_u32()
	}

	fn next_u64(&mut self) -> u64 {
		self.revealed = true;
		self.rng.next_u64()
	}

	fn fill_bytes(&mut self, buf: &mut [u8]) {
		self.revealed = true;
		self.rng.fill_bytes(buf)
	}

	fn try_fill_bytes(&mut self, buf: &mut [u8])
			-> Result<(), rand::Error> {
		self.revealed = true;
		self.rng.try_fill_bytes(buf)
	}
}

pub struct Replay {
	frames: Vec<Frame>,
	infos: Vec<Info>,
	history: Vec<Game>,

	current_frame: Frame,
	current_info: Info,
	game: Game,
}

impl Replay {
	pub fn new(game: Game) -> Replay {
		let info = Info::new();
		Replay{
			frames: Vec::new(),
			infos: vec![info.clone()],
			history: vec![game.clone()],

			current_frame: Frame::new(),
			current_info: info,
			game: game,
		}
	}

	pub fn get_game(&self) -> &Game {
		&self.game
	}

	pub fn get_frame_num(&self) -> usize {
		self.history.len() - 1
	}

	pub fn frame_is_empty(&self) -> bool {
		self.current_frame.actions.len() == 0
	}

	pub fn clear_current(&mut self) -> bool {
		if self.frame_is_empty() {
			return false;
		}
		self.current_frame.actions.clear();
		self.game = self.history.last()
			.expect("there should be a previous state")
			.clone();
		true
	}

	pub fn undo(&mut self) -> bool {
		let num = self.get_frame_num();
		if num == 0 {
			return false;
		}
		self.history.pop();
		self.current_frame = Frame::new();
		self.current_info = self.infos[num - 1].clone();
		self.game = self.history.last()
			.expect("there should be a previous state")
			.clone();
		true
	}

	pub fn clear_or_undo(&mut self) -> bool {
		self.clear_current() || self.undo()
	}

	pub fn redo(&mut self) -> bool {
		let num = self.get_frame_num();
		if num == self.frames.len() {
			return false;
		}
		let backup = self.game.clone();
		self.game = self.history.last()
			.expect("there should be a previous state")
			.clone();
		let actions = &self.frames[num].actions;
		for (i, action) in actions.iter()
			.enumerate() {
			let expected = i == actions.len() - 1;
			let actual = self.game.update(
				*action, &mut self.current_info);
			if expected != actual {
				self.current_info = self.infos[num].clone();
				self.game = backup.clone();
				return false;
			}
		}
		self.current_frame = Frame::new();
		self.current_info = self.infos[num + 1].clone();
		self.history.push(self.game.clone());
		true
	}

	pub fn add_action(&mut self, action: Action) {
		self.current_frame.actions.push(action);
		if self.game.update(action, &mut self.current_info) {
			let num = self.get_frame_num();
			let completed_frame = std::mem::replace(
				&mut self.current_frame, Frame::new());
			if self.frames.len() == num {
				self.frames.push(completed_frame);
				self.infos.push(Info::new());
			} else {
				self.frames[num] = completed_frame;
			}
			self.current_frame = Frame::new();
			self.current_info = self.infos[num + 1].clone();
			self.history.push(self.game.clone());
		}
	}

	pub fn reroll_next_info(&mut self) {
		self.current_info = Info::new();
		let num = self.get_frame_num();
		self.infos[num] = self.current_info.clone();
	}

	pub fn reroll_prev_info(&mut self, back: usize) -> bool {
		let num = self.get_frame_num();
		if back > num {
			return false;
		}
		for _i in 0..back {
			self.undo();
		}
		self.reroll_next_info();
		(0..back).all(|_| self.redo());
		true
	}
}