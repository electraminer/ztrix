use crate::game::game::Clear;
use std::collections::HashMap;
use rand::RngCore;

use crate::game::Action;

use crate::game::Game;

use rand::prelude::*;

#[derive(Clone)]
pub struct Info {
	index: usize,
	info: Vec<u32>,
	generator: ThreadRng,
}

impl Info {
	pub fn new() -> Info {
		Info {
			index: 0,
			info: Vec::new(),
			generator: rand::thread_rng(),
		}
	}

	pub fn next_u32(&mut self) -> u32 {
		if self.info.len() == self.index {
			self.info.push(self.generator.next_u32());
		}
		let info = self.info[self.index];
		self.index += 1;
		info
	}

	pub fn choice<'a, T>(&mut self, options: &'a Vec<T>) -> &'a T {
		let choice = (self.next_u32() as usize) % options.len();
		&options[choice]
	}
}

pub struct Replay {
	current: Vec<Action>,
	choices: HashMap<Game, Vec<Action>>,
	game: Game,
	game_history: Vec<Game>,
	info: Info,
	info_history: Vec<usize>,
}

impl Replay {
	pub fn new(game: Game) -> Self {
		let info = Info::new();
		let index = info.index;
		let mut replay = Self {
			current: Vec::new(),
			choices: HashMap::new(),
			game: game.clone(),
			game_history: vec![game],
			info: info,
			info_history: vec![index],
		};
		replay.update(Action::Init);
		replay.new_frame();
		replay
	}

	pub fn get_game(&self) -> &Game {
		&self.game
	}

	pub fn get_frame(&self) -> usize {
		self.game_history.len() - 1
	}

	pub fn get_num_revealed(&self) -> usize {
		self.info.index
	}

	pub fn revert(&mut self) {
		self.current.clear();
		self.game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
	}

	pub fn new_frame(&mut self) {
		if self.current.len() == 0 {
			return;
		}
		let game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
		let choice = std::mem::replace(
			&mut self.current, Vec::new());
		self.choices.insert(game, choice);
		self.game_history.push(self.game.clone());
		self.info_history.push(self.info.index);
	}

	pub fn undo(&mut self) {
		if self.get_frame() == 0 {
			return;
		}
		self.current.clear();
		self.game_history.pop();
		self.game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
		self.info_history.pop();
		self.info.index = self.info_history.last()
			.expect("there should be a previous state")
			.clone();
	}

	pub fn redo(&mut self) -> Result<Vec<Clear>, ()> {
		let game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
		if let Some(choice) = self.choices.get(&game) {
			let mut clears = Vec::new();
			self.current.clear();
			self.game = game;
			for action in choice.iter() {
				clears.append(&mut
					self.game.update(*action, &mut self.info));
			}
			self.game_history.push(self.game.clone());
			self.info_history.push(self.info.index);
			Ok(clears)
		} else {
			Err(())
		}
	}

	pub fn update(&mut self, action: Action)
		-> Vec<Clear> {
		self.current.push(action);
		let clears = self.game.update(action, &mut self.info);
		let index = self.info_history.last()
			.expect("there should be a previous state")
			.clone();
		if self.info.index != index {
			self.new_frame();
		}
		clears
	}

	pub fn reroll_forward(&mut self, forward: usize) {
		self.info.info[self.info.index + forward] += 1;
	}

	pub fn reroll_backward(&mut self, backward: usize) {
		if backward > self.info.index {
			return;
		}
		let target_index = self.info.index - backward;
		let mut choices = Vec::new();
		while self.info.index > target_index {
			self.undo();
			choices.push(self.choices.get(&self.game)
				.expect("should have saved actions")
				.clone());
		}
		self.reroll_forward(target_index - self.info.index);
		for choice in choices.iter().rev() {
			for action in choice.iter() {
				self.update(*action);
			}
			self.new_frame();
		}
	}
}