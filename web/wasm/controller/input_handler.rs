use std::collections::HashMap;
use std::hash::Hash;
use instant::Duration;

use instant::Instant;

pub enum ButtonEvent<B> {
	Press(B),
	Release(B),
}

impl<B> ButtonEvent<B> {
	pub fn map<F, U>(self, func: F) -> ButtonEvent<U>
	where	F: Fn(B) -> U {
		match self {
			ButtonEvent::Press(b) =>
				ButtonEvent::Press(func(b)),
			ButtonEvent::Release(b) =>
				ButtonEvent::Release(func(b)),
		}
	}
}

impl<B> ButtonEvent<B> {
	pub fn maybe_map<F, U>(self, func: F)
			-> Option<ButtonEvent<U>>
	where	F: Fn(B) -> Option<U> {
		match self {
			ButtonEvent::Press(b) => match func(b) {
				None => None,
				Some(b) => Some(ButtonEvent::Press(b)),
			}
			ButtonEvent::Release(b) => match func(b) {
				None => None,
				Some(b) => Some(ButtonEvent::Release(b)),
			}
		}
	}
}

pub enum InputEvent<B> {
	Button(ButtonEvent<B>),
	PassTime(Duration),
}

pub struct InputHandler<B>
where	B: Copy + Hash + Eq {
	pressed_count: HashMap<B, u32>,
	start_time: Instant,
	time: Duration,
}

impl<B> InputHandler<B>
where	B: Copy + Hash + Eq {
	pub fn new() -> Self {
		Self {
			pressed_count: HashMap::new(),
			start_time: Instant::now(),
			time: Duration::ZERO,
		}
	}

	pub fn button_event(&mut self, event: ButtonEvent<B>)
			-> Option<InputEvent<B>> {
    	match event {
    		ButtonEvent::Press(button) => {
    			let count = *self.pressed_count.get(&button)
    				.unwrap_or(&0);
				self.pressed_count.insert(button, count + 1);
				(count == 0).then_some(
					InputEvent::Button(event))
			},
    		ButtonEvent::Release(button) => {
    			let count = *self.pressed_count.get(&button)
    				.unwrap_or(&0);
    			self.pressed_count.insert(button, match count {
    				0 => 0,
    				c => c - 1,
    			});
				(count == 1).then_some(
					InputEvent::Button(event))

			},
		}
	}

	pub fn time_passed(&mut self)
		-> InputEvent<B> {
		let prev_time = self.time;
		self.time = self.start_time.elapsed();
		let time_elapsed = self.time - prev_time;
		InputEvent::PassTime(time_elapsed)
	}
}