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
	pub fn new(game: Game) -> Replay {
		let info = Info::new();
		let index = info.index;
		Replay{
			current: Vec::new(),
			choices: HashMap::new(),
			game: game.clone(),
			game_history: vec![game],
			info: info,
			info_history: vec![index],
		}
	}

	pub fn get_game(&self) -> &Game {
		&self.game
	}

	pub fn get_frame(&self) -> usize {
		self.game_history.len() - 1
	}

	pub fn revert(&mut self) {
		self.current.clear();
		self.game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
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

	pub fn redo(&mut self) {
		let game = self.game_history.last()
			.expect("there should be a previous state")
			.clone();
		if let Some(choice) = self.choices.get(&game) {
			self.current.clear();
			self.game = game;
			for action in choice.iter() {
				self.game.update(*action, &mut self.info);
			}
			self.game_history.push(self.game.clone());
			self.info_history.push(self.info.index);
		}
	}

	pub fn update(&mut self, action: Action) {
		self.current.push(action);
		self.game.update(action, &mut self.info);
		let index = self.info_history.last()
			.expect("there should be a previous state")
			.clone();
		if self.info.index != index {
			let game = self.game_history.last()
				.expect("there should be a previous state")
				.clone();
			let choice = std::mem::replace(
				&mut self.current, Vec::new());
			self.choices.insert(game, choice);
			self.game_history.push(self.game.clone());
			self.info_history.push(self.info.index);
		}
	}

	pub fn reroll_forward(&mut self, forward: usize) {
		self.info.info[self.info.index + forward] += 1;
	}

	pub fn reroll_backward(&mut self, backward: usize) {
		if backward > self.info.index {
			return;
		}
		let mut actions = Vec::new();
		for _ in 0..backward {
			self.undo();
			let choice = self.choices.get(&self.game)
				.expect("should have saved actions")
				.clone();
			for action in choice.iter().rev() {
				actions.push(*action);
			}
		}
		self.reroll_forward(0);
		for action in actions.iter().rev() {
			self.update(*action);
		}
	}
}