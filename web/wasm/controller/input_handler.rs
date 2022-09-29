use instant::Duration;

use enum_map::EnumMap;
use std::collections::HashSet;
use enum_map::Enum;

use enum_map::enum_map;

use instant::Instant;

#[derive(Debug)]
pub enum InputEvent {
	KeyDown(String),
	KeyUp(String),
	BtnTouchDown(String),
	BtnTouchUp(String),
	BtnClick(String),
	LostFocus,
	GainedFocus,
	TimePassed,
}

pub trait InputBindings<V>
where	V: Copy + Enum + enum_map::EnumArray<u32> + enumset::EnumSetType {
	fn map_key(&self, code: &String) -> Option<V>;
	fn map_button(&self, code: &String) -> Option<V>;
}

#[derive(Copy, Clone, Debug)]
pub enum VirtualInputEvent<V>
where	V: Copy + Enum + enum_map::EnumArray<u32> + enumset::EnumSetType {
	Pressed(V),
	Released(V),
	TimePassed(Duration),
}

pub struct InputHandler<V>
where	V: Copy + Enum + enum_map::EnumArray<u32> + enumset::EnumSetType {
	pressed_keys: HashSet<String>,
	pressed_buttons: HashSet<String>,
	pressed_action_count: EnumMap<V, u32>,
	start_time: Instant,
	time: Duration,
	touched: bool,
	focused: bool,
}

impl<V> InputHandler<V>
where	V: Copy + Enum + enum_map::EnumArray<u32> + enumset::EnumSetType {
	pub fn new() -> InputHandler<V> {
		InputHandler{
			pressed_keys: HashSet::new(),
			pressed_buttons: HashSet::new(),
			pressed_action_count: enum_map! { _ => 0 },
			start_time: Instant::now(),
			time: Duration::ZERO,
			touched: false,
			focused: true,
		}
	}

	fn add_press(&mut self, action: V)
			-> Vec<VirtualInputEvent<V>> {
		let count = self.pressed_action_count[action];
		self.pressed_action_count[action] += 1;
		if count == 0 {
			return vec![VirtualInputEvent::Pressed(action)]
		}
		Vec::new()
	}

	fn remove_press(&mut self, action: V)
			-> Vec<VirtualInputEvent<V>> {
		self.pressed_action_count[action] -= 1;
		let count = self.pressed_action_count[action];
		if count == 0 {
			return vec![VirtualInputEvent::Released(action)]
		}
		Vec::new()
	}

	pub fn update<B>(&mut self, event: InputEvent,
			bindings: &B) -> Vec<VirtualInputEvent<V>>
	where	B: InputBindings<V> {
    	match event {
    		InputEvent::KeyDown(code) => {
    			if let Some(action) = bindings.map_key(&code) {
					if self.pressed_keys.insert(code) {
						return self.add_press(action);
					}
    			}},
    		InputEvent::KeyUp(code) => {
    			if let Some(action) = bindings.map_key(&code) {
	    			if self.pressed_keys.remove(&code) {
	    				return self.remove_press(action);
    				}
    			}},
    		InputEvent::BtnTouchDown(code) => {
    			if let Some(action) = bindings.map_button(&code) {
	    			if self.pressed_buttons.insert(code) {
	    				return self.add_press(action);
	    			}
    			}},
    		InputEvent::BtnTouchUp(code) => {
    			if let Some(action) = bindings.map_button(&code) {
	    			if self.pressed_buttons.remove(&code) {
	    				self.touched = true;
	    				return self.remove_press(action);
	    			}
    			}},
    		InputEvent::BtnClick(code) => {
    			if self.touched {
    				self.touched = false;
    			} else if let Some(action) = bindings.map_button(&code) {
	    			if !self.pressed_buttons.contains(&code) {
	    				let mut vec = self.add_press(action);
	    				vec.append(&mut self.remove_press(action));
	    				return vec;
	    			}
    			}},
    		InputEvent::LostFocus => self.focused = false,
    		InputEvent::GainedFocus => self.focused = true,
    		InputEvent::TimePassed => {
	    		let mut vec = Vec::new();
    			if !self.focused {
	    			let keys: Vec<String> = self.pressed_keys.drain()
	    				.collect();
	    			for code in keys {
	    				if let Some(action) = bindings.map_key(&code) {
	    					let mut v = self.remove_press(action);
	    					vec.append(&mut v);
	    				}
	    			}
    			}
    			let prev_time = self.time;
    			self.time = self.start_time.elapsed();
    			let time_elapsed = self.time - prev_time;
    			vec.push(VirtualInputEvent::TimePassed(time_elapsed));
    			return vec;
    		}
    	}
    	Vec::new()
	}
}