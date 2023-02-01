use crate::game::game::Event;
use crate::puzzle::Puzzle;
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
	choices: HashMap<Puzzle, Vec<Action>>,
	puzzle: Puzzle,
	puzzle_history: Vec<Puzzle>,
	info: Info,
	info_history: Vec<usize>,
}

impl Replay {
	pub fn new<F>(puzzle: Puzzle, event_handler: &mut F) -> Self
    where   F: FnMut(&Event) {
		let info = Info::new();
		let index = info.index;
		let mut replay = Self {
			current: Vec::new(),
			choices: HashMap::new(),
			puzzle: puzzle.clone(),
			puzzle_history: vec![puzzle],
			info: info,
			info_history: vec![index],
		};
		replay.update(Action::Init, event_handler);
		replay.new_frame();
		replay
	}

	pub fn get_puzzle(&self) -> &Puzzle {
		&self.puzzle
	}

	pub fn get_game(&self) -> &Game {
		self.puzzle.get_game()
	}

	pub fn get_frame(&self) -> usize {
		self.puzzle_history.len() - 1
	}

	pub fn get_num_revealed(&self) -> usize {
		self.info.index
	}

	pub fn revert(&mut self) {
		self.current.clear();
		self.puzzle = self.puzzle_history.last()
			.expect("there should be a previous state")
			.clone();
	}

	pub fn new_frame(&mut self) {
		if self.current.len() == 0 {
			return;
		}
		let puzzle = self.puzzle_history.last()
			.expect("there should be a previous state")
			.clone();
		let choice = std::mem::replace(
			&mut self.current, Vec::new());
		self.choices.insert(puzzle, choice);
		self.puzzle_history.push(self.puzzle.clone());
		self.info_history.push(self.info.index);
	}

	pub fn undo(&mut self) {
		if self.get_frame() == 0 {
			return;
		}
		self.current.clear();
		self.puzzle_history.pop();
		self.puzzle = self.puzzle_history.last()
			.expect("there should be a previous state")
			.clone();
		self.info_history.pop();
		self.info.index = self.info_history.last()
			.expect("there should be a previous state")
			.clone();
	}

	pub fn redo<F>(&mut self, event_handler: &mut F) -> bool
    where   F: FnMut(&Event) {
		let puzzle = self.puzzle_history.last()
			.expect("there should be a previous state")
			.clone();
		if let Some(choice) = self.choices.get(&puzzle) {
			self.current.clear();
			self.puzzle = puzzle;
			for action in choice.iter() {
				self.puzzle.update(*action, &mut self.info, event_handler);
			}
			self.puzzle_history.push(self.puzzle.clone());
			self.info_history.push(self.info.index);
			return true;
		}
		false
	}

	pub fn update<F>(&mut self, action: Action, event_handler: &mut F)
    where   F: FnMut(&Event) {
		self.current.push(action);
		let clears = self.puzzle.update(action, &mut self.info, event_handler);
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

	pub fn reroll_backward<F>(&mut self, backward: usize, event_handler: &mut F)
    where   F: FnMut(&Event) {
		if backward > self.info.index {
			return;
		}
		let target_index = self.info.index - backward;
		let mut choices = Vec::new();
		while self.info.index > target_index {
			self.undo();
			choices.push(self.choices.get(&self.puzzle)
				.expect("should have saved actions")
				.clone());
		}
		self.reroll_forward(target_index - self.info.index);
		for choice in choices.iter().rev() {
			for action in choice.iter() {
				self.update(*action, event_handler);
			}
			self.new_frame();
		}
	}

	fn to_solution(&mut self) -> Solution {
		let mut actions = Vec::new();

		let initial = self.puzzle_history.last()
			.expect("there should be a previous state")
			.clone();
		let mut puzzle = initial.clone();

		let mut info = self.info.clone();
		while let Some(choice) = self.choices.get(&puzzle) {
			for action in choice.iter() {
				puzzle.update(*action, &mut info, &mut |_| ());
				actions.push(action.clone());
			}
		}

		Solution {
			puzzle: initial,
			info: self.info.info[self.info.index..].to_vec(),
			actions: actions
		}
	}

	fn from_solution(solution: Solution) -> Self {
		let mut replay = Self::new(solution.puzzle, &mut |_| ());

		replay.info.info = solution.info;

		for action in solution.actions {
			replay.update(action, &mut |_| ());
		}

		replay
	}


}

struct Solution {
	puzzle: Puzzle,
	info: Vec<u32>,
    actions: Vec<Action>,
}