
use controller::input_handler::InputBindings;
use controller::action::Action;

use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct GameInterfaceBindings {
	key_bindings: HashMap<String, Action>,
	button_bindings: HashMap<String, Action>,
	left_buttons: Vec<Action>,
	right_buttons: Vec<Action>,
	bottom_buttons: Vec<Vec<Action>>,
}

impl GameInterfaceBindings {
	pub fn new(key_bindings: HashMap<String, Action>,
			left_buttons: Vec<Action>, right_buttons: Vec<Action>,
			bottom_buttons: Vec<Vec<Action>>) -> GameInterfaceBindings {
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
		GameInterfaceBindings{
			key_bindings: key_bindings,
			button_bindings: button_bindings,
			left_buttons: left_buttons,
			right_buttons: right_buttons,
			bottom_buttons: bottom_buttons,
		}
	}

	pub fn get_key_bindings(&self) -> &HashMap<String, Action> {
		&self.key_bindings
	}

	pub fn get_button_bindings(&self)-> &HashMap<String, Action> {
		&self.button_bindings
	}

	pub fn get_left_buttons(&self) -> &Vec<Action> {
		&self.left_buttons
	}

	pub fn get_right_buttons(&self) -> &Vec<Action> {
		&self.right_buttons
	}

	pub fn get_bottom_buttons(&self) -> &Vec<Vec<Action>> {
		&self.bottom_buttons
	}
}

impl InputBindings<Action> for GameInterfaceBindings {
	fn map_key(&self, code: &String) -> Option<Action> {
		self.key_bindings.get(code).copied()
	}

	fn map_button(&self, code: &String) -> Option<Action> {
		match code.as_str() {
			"RerollNext1" => Some(Action::RerollNext1),
			"RerollNext2" => Some(Action::RerollNext2),
			"RerollNext3" => Some(Action::RerollNext3),
			"RerollNext4" => Some(Action::RerollNext4),
			_ => self.button_bindings.get(code).copied(),
		}
	}
}

impl Default for GameInterfaceBindings {

	fn default() -> GameInterfaceBindings {
		GameInterfaceBindings::new(
			HashMap::from([
				("KeyK", Action::Left),
		    	("Semicolon", Action::Right),
		        ("KeyL", Action::DownSlow),
		        ("ShiftLeft", Action::DownFast),
		        ("KeyO", Action::Clockwise),
		        ("KeyA", Action::Anticlockwise),
		        ("KeyW", Action::Flip),
		        ("Space", Action::Place),
		        ("KeyD", Action::Hold),
		        ("KeyF", Action::Zone),
		        ("KeyZ", Action::Undo),
		        ("KeyR", Action::Redo),
		        ("Backquote", Action::RerollCurrent),
		        ("Digit1", Action::RerollNext1),
		        ("Digit2", Action::RerollNext2),
		        ("Digit3", Action::RerollNext3),
		        ("Digit4", Action::RerollNext4),
		        ("KeyS", Action::Restart)]
		        	.map(|(s, c)| (s.to_string(), c))),
			vec![Action::Restart, Action::Redo],
			vec![Action::RerollCurrent],
			vec![
				vec![
					Action::Undo,
					Action::Place,
					Action::Place,
					Action::Flip],
				vec![
					Action::Left,
					Action::Right,
					Action::Anticlockwise,
					Action::Clockwise],
				vec![
					Action::DownFast,
					Action::DownSlow,
					Action::Zone,
					Action::Hold]])
	}
}