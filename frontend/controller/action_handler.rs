use std::collections::HashSet;
use ztrix::game::Action;
use controller::input_handler::InputEvent;
use controller::input_handler::ButtonEvent;
use ztrix::replay::Replay;
use ztrix::game::MaybeActive;
use user_prefs::UserPrefs;

use std::time::Duration;
use crate::component::play_interface::PlayButton;
use ztrix::position::Rotation;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct HandlingSettings {
    das_duration: Duration,
    arr_duration: Duration,
    down_das_duration: Duration,
    down_arr_duration: Duration,
    entry_delay: Duration,
}

impl Default for HandlingSettings {
	fn default() -> HandlingSettings {
		HandlingSettings{
			das_duration: Duration::from_millis(150),
			arr_duration: Duration::from_millis(30),
			down_das_duration: Duration::from_millis(150),
			down_arr_duration: Duration::from_millis(30),
			entry_delay: Duration::from_millis(100),
		}
	}
}

#[derive(Copy, Clone)]
enum DasDirection {
	Left,
	Right,
	None
}

#[derive(Debug, Copy, Clone)]
pub enum MetaAction {
	Action(Action),
    Revert,
    Undo,
    Redo,
    Reroll(usize),
	Restart,
	Edit,
}

pub struct ActionHandler {
	held: HashSet<PlayButton>,
	das_priority: DasDirection,
	das_timer: Duration,
	down_das_timer: Duration,
	entry_delay_timer: Duration,
	frozen: bool,
	moved: bool,
}

impl ActionHandler {

	pub fn new() -> ActionHandler {
		ActionHandler{
			held: HashSet::new(),
			das_priority: DasDirection::None,
			das_timer: Duration::ZERO,
			down_das_timer: Duration::ZERO,
			entry_delay_timer: Duration::ZERO,
			frozen: true,
			moved: false,
		}
	}

	fn irs(&self) -> Rotation {
		let mut irs = Rotation::Zero;
		if self.held.contains(&PlayButton::Clockwise) {
			irs = irs + Rotation::Clockwise;
		}
		if self.held.contains(&PlayButton::Anticlockwise) {
			irs = irs + Rotation::Anticlockwise;
		}
		if self.held.contains(&PlayButton::Flip) {
			irs = irs + Rotation::Flip;
		}
		irs
	}

	fn das(&self) -> DasDirection {
		let left = self.held.contains(&PlayButton::Left);
		let right = self.held.contains(&PlayButton::Right);
		if !left && !right {
			DasDirection::None
		} else if left && !right {
			DasDirection::Left
		} else if !left && right {
			DasDirection::Right
		} else {
			self.das_priority
		}
	}

	pub fn press(&mut self, replay: &Replay, button: PlayButton)
			-> Vec<MetaAction> {
		let user_prefs = UserPrefs::get();
		let handling_settings = &user_prefs.handling_settings;
		self.held.insert(button);
		self.frozen = false;
		let mut vec = Vec::new();
		let game = replay.get_game();
		if let MaybeActive::Inactive(_) = game.piece {
			if matches!{button, PlayButton::Left | PlayButton::Right |
				PlayButton::DownSlow | PlayButton::Place} {
				vec.push(MetaAction::Action(
					Action::SpawnPiece(
	    				self.irs(),
	    				self.held.contains(&PlayButton::Hold))));
			}
		}
		if matches!{button, PlayButton::Left | PlayButton::Right |
			PlayButton::DownSlow | PlayButton::DownFast | PlayButton::Place
			| PlayButton::Clockwise | PlayButton::Anticlockwise
			| PlayButton::Flip | PlayButton::Zone} {
			self.moved = true;
		}
		if let PlayButton::Hold = button {
			if let Some(_) = game.hold {
				self.moved = true;
			}
		}

		vec.append(&mut match button {
    		PlayButton::Left => {
    			self.das_priority = DasDirection::Left;
    			self.das_timer = handling_settings.das_duration;
    			vec![MetaAction::Action(
    				Action::MoveLeft)]},
		    PlayButton::Right => {
    			self.das_priority = DasDirection::Right;
    			self.das_timer = handling_settings.das_duration;
    			vec![MetaAction::Action(
    				Action::MoveRight)]},
		   	PlayButton::DownSlow => {
    			self.down_das_timer = handling_settings.down_das_duration;
    			vec![MetaAction::Action(
    				Action::MoveDown)]},
		    PlayButton::DownFast => vec![],
		    PlayButton::Clockwise => vec![MetaAction::Action(
		    	Action::Rotate(Rotation::Clockwise))],
		    PlayButton::Anticlockwise => vec![MetaAction::Action(
		    	Action::Rotate(Rotation::Anticlockwise))],
		    PlayButton::Flip => vec![MetaAction::Action(
		    	Action::Rotate(Rotation::Flip))],
		    PlayButton::Place => {
		    	self.entry_delay_timer = handling_settings.entry_delay;
		    	vec![MetaAction::Action(Action::PlacePiece(
    				self.irs(), self.held.contains(&PlayButton::Hold)))]},
	    	PlayButton::Hold => vec![MetaAction::Action(
	    		Action::HoldPiece(self.irs()))],
	    	PlayButton::Zone => vec![MetaAction::Action(
	    		Action::ToggleZone)],
	    	PlayButton::Undo => {
	    		self.frozen = true;
	    		if self.moved {
	    			vec![MetaAction::Revert]
	    		} else {
	    			vec![MetaAction::Undo]
	    		}
	    	}
	    	PlayButton::Redo => {
	    		self.frozen = true;
	    		vec![MetaAction::Redo]
	    	}
	    	PlayButton::RerollCurrent => {
	    		if replay.get_frame() >= 4 + 1 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(4 + 1)]
	    	}
	    	PlayButton::RerollNext(n) => {
	    		if replay.get_frame() >= 4 - n + 1 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(4 - n + 1)]
	    	}
	    	PlayButton::Restart => {
	    		self.frozen = true;
	    		vec![MetaAction::Restart]
	    	}
	    	_ => vec![],
		});
		if self.frozen {
			self.moved = false;
		}
		vec
	}

	pub fn release(&mut self, _replay: &Replay, button: PlayButton)
			 -> Vec<MetaAction> {
		self.held.remove(&button);
		match button {
	    	PlayButton::Edit => {
	    		self.frozen = true;
	    		self.moved = false;
	    		vec![MetaAction::Edit]
	    	}
	    	_ => vec![],
		}
	}

	pub fn pass_time(&mut self, replay: &Replay, duration: Duration)
			 -> Vec<MetaAction> {
		let game = replay.get_game();
		if self.frozen {
			return vec![];
		}
		let user_prefs = UserPrefs::get();
		let handling_settings = &user_prefs.handling_settings;
		let mut vec = Vec::new();

		if self.entry_delay_timer < duration {
			if let MaybeActive::Inactive(_) = game.piece {
				vec.push(MetaAction::Action(
					Action::SpawnPiece(
    					self.irs(), self.held.contains(&PlayButton::Hold))));
				self.moved = false;
			}
		} else {
			self.entry_delay_timer -= duration;
		}

		let mut max_iter = 20;
		while self.down_das_timer < duration {
			self.down_das_timer += handling_settings.down_arr_duration;
			if self.held.contains(&PlayButton::DownSlow) {
				vec.push(MetaAction::Action(
    				Action::MoveDown));
				self.moved = true;
			}
			max_iter -= 1;
			if max_iter == 0 {
				self.down_das_timer = duration;
			}
		}
		self.down_das_timer -= duration;
		if self.held.contains(&PlayButton::DownFast) {
			vec.append(&mut vec![MetaAction::Action(
	   			Action::MoveDown); 20]);
			self.moved = true;
		}

		let mut max_iter = 10;
		while self.das_timer < duration {
			self.das_timer += handling_settings.arr_duration;
			match self.das() {
				DasDirection::None => (),
				DasDirection::Left => {
					vec.push(MetaAction::Action(
    					Action::MoveLeft));
					self.moved = true;
				}
				DasDirection::Right => {
					vec.push(MetaAction::Action(
    					Action::MoveRight));
					self.moved = true;
				}
			}
			max_iter -= 1;
			if max_iter == 0 {
				self.das_timer = duration;
			}
		}
		self.das_timer -= duration;
		vec
	}

	pub fn update(&mut self, replay: &Replay,
		event: InputEvent<PlayButton>) -> Vec<MetaAction> {
		match event {
			InputEvent::Button(
				ButtonEvent::Press(button)) =>
					self.press(replay, button),
			InputEvent::Button(
				ButtonEvent::Release(button)) =>
					self.release(replay, button),
			InputEvent::PassTime(duration) =>
				self.pass_time(replay, duration),
		}
	}
}