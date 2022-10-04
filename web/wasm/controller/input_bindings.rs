
use component::edit_interface::EditButton;
use crate::component::play_interface::PlayButton;

use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBindings {
	pub play_bindings: HashMap<String, PlayButton>,
	pub edit_bindings: HashMap<String, EditButton>,
}

impl KeyBindings {
	pub fn new(play_bindings: HashMap<String, PlayButton>,
			edit_bindings: HashMap<String, EditButton>)
			-> Self {
		Self {
			play_bindings: play_bindings,
			edit_bindings: edit_bindings,
		}
	}
}

impl Default for KeyBindings {
	fn default() -> Self {
		Self::new(
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
		        ("KeyS", PlayButton::Restart),
				("KeyE", PlayButton::Edit),
		    ].map(|(s, c)| (s.to_string(), c))),
			HashMap::from([
				("KeyH", EditButton::SetHold),
				("Backquote", EditButton::SetCurrent),
				("Digit1", EditButton::SetNext(1)),
				("Digit2", EditButton::SetNext(2)),
				("Digit3", EditButton::SetNext(3)),
				("Digit4", EditButton::SetNext(4)),
				("KeyB", EditButton::SetBagPos),
				("KeyF", EditButton::ToggleZone),
				("KeyD", EditButton::ToggleHoldUsed),
				("KeyE", EditButton::Play),
				("KeyV", EditButton::Import),
				("KeyC", EditButton::Export),
				("KeyZ", EditButton::Revert),
				("KeyS", EditButton::EraseAll),
			].map(|(s, c)| (s.to_string(), c))))
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonBindings {
	pub left_buttons: Vec<PlayButton>,
	pub right_buttons: Vec<PlayButton>,
	pub bottom_buttons: Vec<Vec<PlayButton>>,
}

impl ButtonBindings {
	pub fn new(left_buttons: Vec<PlayButton>,
			right_buttons: Vec<PlayButton>,
			bottom_buttons: Vec<Vec<PlayButton>>) -> Self {
		Self {
			left_buttons: left_buttons,
			right_buttons: right_buttons,
			bottom_buttons: bottom_buttons,
		}
	}
}

impl Default for ButtonBindings {
	fn default() -> Self {
		Self::new(
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