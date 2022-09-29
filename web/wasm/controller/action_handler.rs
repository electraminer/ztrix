use ztrix::replay::Replay;
use user_prefs::UserPrefs;
use controller::action::MetaAction;
use enumset::EnumSet;
use controller::input_handler::VirtualInputEvent;
use std::time::Duration;

use ztrix::position::Rotation;

use controller::action::Action;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct HandlingSettings {
    das_duration: Duration,
    arr_duration: Duration,
    down_das_duration: Duration,
    down_arr_duration: Duration,
    entry_delay: Duration,
    reset_duration: Duration,
}

impl Default for HandlingSettings {
	fn default() -> HandlingSettings {
		HandlingSettings{
			das_duration: Duration::from_millis(150),
			arr_duration: Duration::from_millis(30),
			down_das_duration: Duration::from_millis(150),
			down_arr_duration: Duration::from_millis(30),
			entry_delay: Duration::from_millis(100),
			reset_duration: Duration::ZERO,
		}
	}
}

#[derive(Copy, Clone)]
enum DasDirection {
	Left,
	Right,
	None
}

pub struct ActionHandler {
	held: EnumSet<Action>,
	das_priority: DasDirection,
	das_timer: Duration,
	down_das_timer: Duration,
	entry_delay_timer: Duration,
	reset_timer: Duration,
	frozen: bool,
	moved: bool,
}

impl ActionHandler {

	pub fn new() -> ActionHandler {
		ActionHandler{
			held: EnumSet::empty(),
			das_priority: DasDirection::None,
			das_timer: Duration::ZERO,
			down_das_timer: Duration::ZERO,
			entry_delay_timer: Duration::ZERO,
			reset_timer: Duration::ZERO,
			frozen: false,
			moved: false,
		}
	}

	fn irs(&self) -> Rotation {
		let mut irs = Rotation::Zero;
		if self.held.contains(Action::Clockwise) {
			irs = irs + Rotation::Clockwise;
		}
		if self.held.contains(Action::Anticlockwise) {
			irs = irs + Rotation::Anticlockwise;
		}
		if self.held.contains(Action::Flip) {
			irs = irs + Rotation::Flip;
		}
		irs
	}

	fn das(&self) -> DasDirection {
		let left = self.held.contains(Action::Left);
		let right = self.held.contains(Action::Right);
		if !left && !right {
			DasDirection::None
		} else if left && !right {
			DasDirection::Left
		} else if left && !right {
			DasDirection::Right
		} else {
			self.das_priority
		}
	}

	pub fn press(&mut self, replay: &Replay, action: Action)
			-> Vec<MetaAction> {
		let user_prefs = UserPrefs::get();
		let handling_settings = user_prefs.get_handling_settings();
		self.held.insert(action);
		self.frozen = false;
		let mut vec = Vec::new();
		let game = replay.get_game();
		if let None = game.get_piece() {
			if matches!{action, Action::Left | Action::Right |
				Action::DownSlow | Action::Place} {
				vec.push(MetaAction::Action(
					ztrix::game::Action::SpawnPiece(
	    				self.irs(),
	    				self.held.contains(Action::Hold))));
			}
		}
		if matches!{action, Action::Left | Action::Right |
			Action::DownSlow | Action::DownFast | Action::Place
			| Action::Clockwise | Action::Anticlockwise
			| Action::Flip | Action::Hold | Action::Zone} {
			self.moved = true;
		}
		vec.append(&mut match action {
    		Action::Left => {
    			self.das_priority = DasDirection::Left;
    			self.das_timer = handling_settings.das_duration;
    			vec![MetaAction::Action(
    				ztrix::game::Action::MoveLeft)]},
		    Action::Right => {
    			self.das_priority = DasDirection::Right;
    			self.das_timer = handling_settings.das_duration;
    			vec![MetaAction::Action(
    				ztrix::game::Action::MoveRight)]},
		   	Action::DownSlow => {
    			self.down_das_timer = handling_settings.down_das_duration;
    			vec![MetaAction::Action(
    				ztrix::game::Action::MoveDown)]},
		    Action::DownFast => vec![],
		    Action::Clockwise => vec![MetaAction::Action(
		    	ztrix::game::Action::Rotate(Rotation::Clockwise))],
		    Action::Anticlockwise => vec![MetaAction::Action(
		    	ztrix::game::Action::Rotate(Rotation::Anticlockwise))],
		    Action::Flip => vec![MetaAction::Action(
		    	ztrix::game::Action::Rotate(Rotation::Flip))],
		    Action::Place => {
		    	self.entry_delay_timer = handling_settings.entry_delay;
		    	vec![MetaAction::Action(ztrix::game::Action::PlacePiece(
    				self.irs(), self.held.contains(Action::Hold)))]},
	    	Action::Hold => vec![MetaAction::Action(
	    		ztrix::game::Action::HoldPiece(self.irs()))],
	    	Action::Zone => vec![MetaAction::Action(
	    		ztrix::game::Action::ToggleZone)],
	    	Action::Undo => {
	    		self.frozen = true;
	    		if self.moved {
	    			vec![MetaAction::Revert]
	    		} else {
	    			vec![MetaAction::Undo]
	    		}
	    	}
	    	Action::Redo => {
	    		self.frozen = true;
	    		vec![MetaAction::Redo]
	    	}
	    	Action::RerollCurrent => {
	    		if replay.get_frame() >= 5 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(5)]
	    	}
	    	Action::RerollNext1 => {
	    		if replay.get_frame() >= 4 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(4)]
	    	}
	    	Action::RerollNext2 => {
	    		if replay.get_frame() >= 3 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(3)]
	    	}
	    	Action::RerollNext3 => {
	    		if replay.get_frame() >= 2 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(2)]
	    	}
	    	Action::RerollNext4 => {
	    		if replay.get_frame() >= 1 {
	    			self.frozen = true;
	    		}
	    		vec![MetaAction::Reroll(1)]
	    	}
	    	Action::Restart => {
	    		self.reset_timer = handling_settings.reset_duration;
	    		if self.reset_timer == Duration::ZERO {
	    			self.frozen = true;
	    			vec![MetaAction::Restart]
	    		} else {
	    			vec![]
	    		}
	    	}
		});
		if self.frozen {
			self.moved = false;
		}
		vec
	}

	pub fn release(&mut self, _replay: &Replay, action: Action)
			 -> Vec<MetaAction> {
		self.held.remove(action);
		vec![]
	}

	pub fn pass_time(&mut self, replay: &Replay, duration: Duration)
			 -> Vec<MetaAction> {
		let game = replay.get_game();
		if self.frozen {
			return vec![];
		}
		let user_prefs = UserPrefs::get();
		let handling_settings = user_prefs.get_handling_settings();
		let mut vec = Vec::new();
		if self.reset_timer < duration {
			if self.held.contains(Action::Restart) {
	    		self.frozen = true;
				vec.push(MetaAction::Restart);
			}
		} else {
			self.reset_timer -= duration;
		}

		if self.entry_delay_timer < duration {
			if let None = game.get_piece() {
				vec.push(MetaAction::Action(
					ztrix::game::Action::SpawnPiece(
    					self.irs(), self.held.contains(Action::Hold))));
				self.moved = false;
			}
		} else {
			self.entry_delay_timer -= duration;
		}

		let mut max_iter = 20;
		while self.down_das_timer < duration {
			self.down_das_timer += handling_settings.down_arr_duration;
			if self.held.contains(Action::DownSlow) {
				vec.push(MetaAction::Action(
    				ztrix::game::Action::MoveDown));
			}
			max_iter -= 1;
			if max_iter == 0 {
				self.down_das_timer = duration;
			}
		}
		self.down_das_timer -= duration;
		if self.held.contains(Action::DownFast) {
			vec.append(&mut vec![MetaAction::Action(
	   			ztrix::game::Action::MoveDown); 20]);
		}

		let mut max_iter = 10;
		while self.das_timer < duration {
			self.das_timer += handling_settings.arr_duration;
			match self.das() {
				DasDirection::None => (),
				DasDirection::Left => vec.push(MetaAction::Action(
    				ztrix::game::Action::MoveLeft)),
				DasDirection::Right => vec.push(MetaAction::Action(
    				ztrix::game::Action::MoveRight)),
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
		event: VirtualInputEvent<Action>) -> Vec<MetaAction> {
		match event {
			VirtualInputEvent::Pressed(action) =>
				self.press(replay, action),
			VirtualInputEvent::Released(action) =>
				self.release(replay, action),
			VirtualInputEvent::TimePassed(duration) =>
				self.pass_time(replay, duration),
		}
	}
}