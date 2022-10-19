
use ztrix::game::game::Clear;
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
#[derive(Clone, Debug)]
pub enum IrsMode {
	Accurate,
	Lenient,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct HandlingSettings {
    pub irs_mode: IrsMode,
    pub das_duration: Duration,
    pub arr_duration: Duration,
    pub down_das_duration: Duration,
    pub down_arr_duration: Duration,
    pub entry_delay: Duration,
    pub buffer_delay: Duration,
}

impl Default for HandlingSettings {
	fn default() -> HandlingSettings {
		HandlingSettings{
			irs_mode: IrsMode::Lenient,
			das_duration: Duration::from_millis(150),
			arr_duration: Duration::from_millis(30),
			down_das_duration: Duration::from_millis(150),
			down_arr_duration: Duration::from_millis(30),
			entry_delay: Duration::from_millis(100),
			buffer_delay: Duration::from_millis(20),
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
	held: HashSet<PlayButton>,
	das_priority: DasDirection,
	das_timer: Duration,
	down_das_timer: Duration,
	entry_delay_timer: Duration,
	frozen: bool,
	moved: bool,
	pub last_zone_clear: Option<Clear>,
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
			last_zone_clear: None,
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

	fn ihs(&self) -> bool {
		self.held.contains(&PlayButton::Hold)
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

	fn spawn(&mut self, replay: &mut Replay) {
		let clears = replay.update(Action::SpawnPiece(
	    	self.irs(), self.ihs()));
		if let Some(Clear::ZoneClear(_)) = clears.last() {
			self.last_zone_clear = clears.last().cloned();
			self.moved = true;
		}
		if replay.get_game().over {
			self.moved = true;
		}
	}

	pub fn press(&mut self, replay: &mut Replay,
			button: PlayButton) {
		let user_prefs = UserPrefs::get();
		let handling_settings = &user_prefs.handling_settings;
		self.held.insert(button);

		self.frozen = false;
		if let Some(MaybeActive::Inactive(_)) = replay.get_game().piece {
			if matches!{button, PlayButton::Left | PlayButton::Right |
				PlayButton::DownSlow | PlayButton::Place} {
				self.spawn(replay);
			}
		}
		if matches!{button, PlayButton::Clockwise
			| PlayButton::Anticlockwise | PlayButton::Flip
			| PlayButton::Hold} {
    		self.entry_delay_timer = handling_settings.buffer_delay;
		}

		if matches!{button, PlayButton::Left | PlayButton::Right |
			PlayButton::DownSlow | PlayButton::DownFast | PlayButton::Place
			| PlayButton::Clockwise | PlayButton::Anticlockwise
			| PlayButton::Flip | PlayButton::Zone} {
			self.moved = true;
		}

		if let PlayButton::Hold = button {
			if let Some(_) = replay.get_game().hold {
				self.moved = true;
			}
		}

		match button {
    		PlayButton::Left => {
    			self.das_priority = DasDirection::Left;
    			self.das_timer = handling_settings.das_duration;
    			replay.update(Action::MoveLeft);
    		}
		    PlayButton::Right => {
    			self.das_priority = DasDirection::Right;
    			self.das_timer = handling_settings.das_duration;
    			replay.update(Action::MoveRight);
    		}
		   	PlayButton::DownSlow => {
    			self.down_das_timer = handling_settings.down_das_duration;
    			replay.update(Action::MoveDown);
    		}
		    PlayButton::DownFast =>
				for _ in 0..26 {
					replay.update(Action::MoveDown);
				}
		    PlayButton::Clockwise => {
		    	replay.update(Action::Rotate(
    				Rotation::Clockwise));
		    }
		    PlayButton::Anticlockwise => {
		    	replay.update(Action::Rotate(
		    		Rotation::Anticlockwise));
		    }
		    PlayButton::Flip => {
		    	replay.update(Action::Rotate(Rotation::Flip));
		    }
		    PlayButton::Place => {
		    	if !replay.get_game().over {
			    	replay.update(Action::PlacePiece);
			    	replay.new_frame();
			    	self.entry_delay_timer = handling_settings.entry_delay;
					self.moved = false;

					if let IrsMode::Accurate = handling_settings.irs_mode {	
						self.spawn(replay);
						if !self.moved {
							replay.revert();
						}
					}
		    	}
		    }
	    	PlayButton::Hold => {
	    		if !replay.get_game().over {
		    		if replay.get_game().hold == None {
			    		replay.new_frame();
		    		}
		    		replay.update(Action::HoldPiece(self.irs()));
	    		}
	    	}
	    	PlayButton::Zone => {
	    		let clears = replay.update(Action::ToggleZone);
	    		self.last_zone_clear = clears.last().cloned();
	    	}
	    	PlayButton::Undo => {
	    		if self.moved || replay.get_frame() <= 1 {
	    			replay.revert();
	    		} else {
	    			replay.undo();
	    			self.last_zone_clear = None;
	    		}
	    		self.entry_delay_timer = handling_settings.entry_delay;
	    		self.frozen = true;
				self.moved = false;
	    	}
	    	PlayButton::Redo => {
	    		if let Ok(clears) = replay.redo() {
	    			if let Some(Clear::ZoneClear(_)) = clears.last() {
	    				self.last_zone_clear = clears.last().cloned();
	    			}
	    		}
	    		self.entry_delay_timer = handling_settings.entry_delay;
	    		self.frozen = true;
				self.moved = false;
	    	}
	    	PlayButton::RerollCurrent =>
	    		if replay.get_frame() >= 4 + 1 {
    				replay.reroll_backward(4 + 1);
	    		self.entry_delay_timer = handling_settings.entry_delay;
	    			self.frozen = true;
					self.moved = false;
	    		}
	    	PlayButton::RerollNext(n) =>
	    		if replay.get_frame() >= 4 - n + 1 {
    				replay.reroll_backward(4 - n + 1);
	    		self.entry_delay_timer = handling_settings.entry_delay;
	    			self.frozen = true;
					self.moved = false;
	    		}
	    	PlayButton::Restart => {
	    		replay.revert();
				for _ in 1..replay.get_frame() {
					replay.undo();
				}
	    		self.last_zone_clear = None;
	    		self.entry_delay_timer = handling_settings.entry_delay;
	    		self.frozen = true;
				self.moved = false;
	    	}
	    	_ => (),
		}
	}

	pub fn release(&mut self, _replay: &mut Replay,
			button: PlayButton) {
		self.held.remove(&button);
	}

	pub fn pass_time(&mut self, replay: &mut Replay,
			duration: Duration) {
		let game = replay.get_game();
		if self.frozen {
			return;
		}
		let user_prefs = UserPrefs::get();
		let handling_settings = &user_prefs.handling_settings;

		if self.entry_delay_timer < duration {
			if let Some(MaybeActive::Inactive(_)) = game.piece {
				self.spawn(replay);
			}
		} else {
			self.entry_delay_timer -= duration;
		}

		let mut max_iter = 10;
		while self.das_timer < duration {
			self.das_timer += handling_settings.arr_duration;
			match self.das() {
				DasDirection::None => (),
				DasDirection::Left => {
					replay.update(Action::MoveLeft);
					self.moved = true;
				}
				DasDirection::Right => {
					replay.update(Action::MoveRight);
					self.moved = true;
				}
			}
			max_iter -= 1;
			if max_iter == 0 {
				self.das_timer = duration;
			}
		}
		self.das_timer -= duration;

		let mut max_iter = 26;
		while self.down_das_timer < duration {
			self.down_das_timer += handling_settings.down_arr_duration;
			if self.held.contains(&PlayButton::DownSlow) {
				replay.update(Action::MoveDown);
				self.moved = true;
			}
			max_iter -= 1;
			if max_iter == 0 {
				self.down_das_timer = duration;
			}
		}
		self.down_das_timer -= duration;
		if self.held.contains(&PlayButton::DownFast) {
			for _ in 0..26 {
				replay.update(Action::MoveDown);
			}
			self.moved = true;
		}
	}

	pub fn update(&mut self, replay: &mut Replay,
		event: InputEvent<PlayButton>) {
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