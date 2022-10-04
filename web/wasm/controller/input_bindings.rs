
use crate::component::play_interface::PlayButton;

use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct InputBindings {
	play_key_bindings: HashMap<String, PlayButton>,
	button_bindings: HashMap<String, PlayButton>,
	left_buttons: Vec<PlayButton>,
	right_buttons: Vec<PlayButton>,
	bottom_buttons: Vec<Vec<PlayButton>>,
}

impl InputBindings {
	pub fn new(play_key_bindings: HashMap<String, PlayButton>,
			left_buttons: Vec<PlayButton>, right_buttons: Vec<PlayButton>,
			bottom_buttons: Vec<Vec<PlayButton>>) -> InputBindings {
		let mut button_bindings = HashMap::new();
		for (i, action) in left_buttons.iter().enumerate() {
			let code = format!{"LeftButtons[{}]", i};
			button_bindings.insert(code, *action);
		}
		for (i, action) in right_buttons.iter().enumerate() {
			let code = format!{"RightButtons[{}]", i};
			button_bindings.insert(code, *action);
		}
		for (i, row) in bottom_buttons.iter().enumerate() {
			for (j, action) in row.iter().enumerate() {
				let code = format!{"BottomButtons[{}][{}]", i, j};
				button_bindings.insert(code, *action);
			}
		}
		InputBindings{
			play_key_bindings: play_key_bindings,
			button_bindings: button_bindings,
			left_buttons: left_buttons,
			right_buttons: right_buttons,
			bottom_buttons: bottom_buttons,
		}
	}

	pub fn get_play_key_bindings(&self) -> &HashMap<String, PlayButton> {
		&self.play_key_bindings
	}

	pub fn get_button_bindings(&self)-> &HashMap<String, PlayButton> {
		&self.button_bindings
	}

	pub fn get_left_buttons(&self) -> &Vec<PlayButton> {
		&self.left_buttons
	}

	pub fn get_right_buttons(&self) -> &Vec<PlayButton> {
		&self.right_buttons
	}

	pub fn get_bottom_buttons(&self) -> &Vec<Vec<PlayButton>> {
		&self.bottom_buttons
	}

	pub fn map_key_to_play(&self, code: &String)
			-> Option<PlayButton> {
		self.play_key_bindings.get(code).copied()
	}
}

impl Default for InputBindings {

	fn default() -> InputBindings {
		InputBindings::new(
			HashMap::from([
				("KeyK", PlayButton::Left),
		    	("Semicolon", PlayButton::Right),
		        ("KeyL", PlayButton::DownSlow),
		        ("ShiftLeft", PlayButton::DownFast),
		        ("KeyO", PlayButton::Clockwise),
		        ("KeyA", PlayButton::Anticlockwise),
		        ("KeyW", PlayButton::Flip),
		        ("Space", PlayButton::Place),
		        ("KeyD", PlayButton::Hold),
		        ("KeyF", PlayButton::Zone),
		        ("KeyZ", PlayButton::Undo),
		        ("KeyR", PlayButton::Redo),
		        ("Backquote", PlayButton::RerollCurrent),
		        ("Digit1", PlayButton::RerollNext(1)),
		        ("Digit2", PlayButton::RerollNext(2)),
		        ("Digit3", PlayButton::RerollNext(3)),
		        ("Digit4", PlayButton::RerollNext(4)),
		        ("KeyS", PlayButton::Restart)]
		        	.map(|(s, c)| (s.to_string(), c))),
			vec![PlayButton::Restart, PlayButton::Redo],
			vec![PlayButton::RerollCurrent],
			vec![
				vec![
					PlayButton::Undo,
					PlayButton::Place,
					PlayButton::Place,
					PlayButton::Flip],
				vec![
					PlayButton::Left,
					PlayButton::Right,
					PlayButton::Anticlockwise,
					PlayButton::Clockwise],
				vec![
					PlayButton::DownFast,
					PlayButton::DownSlow,
					PlayButton::Zone,
					PlayButton::Hold]])
	}
}