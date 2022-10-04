use std::collections::HashMap;
use std::hash::Hash;
use instant::Duration;

use instant::Instant;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum InputEvent<B> {
	Button(ButtonEvent<B>),
	PassTime(Duration),
}

pub struct ButtonHandler<B>
where	B: Copy + Hash + Eq {
	pressed_count: HashMap<B, u32>,
}

impl<B> ButtonHandler<B>
where	B: Copy + Hash + Eq {
	pub fn new() -> Self {
		Self {
			pressed_count: HashMap::new(),
		}
	}

	pub fn update(&mut self, event: ButtonEvent<B>)
			-> Option<ButtonEvent<B>> {
    	match event {
    		ButtonEvent::Press(button) => {
    			let count = *self.pressed_count.get(&button)
    				.unwrap_or(&0);
				self.pressed_count.insert(button, count + 1);
				(count == 0).then_some(event)
			},
    		ButtonEvent::Release(button) => {
    			let count = *self.pressed_count.get(&button)
    				.unwrap_or(&0);
    			self.pressed_count.insert(button, match count {
    				0 => 0,
    				c => c - 1,
    			});
				(count == 1).then_some(event)

			},
		}
	}
}

pub struct TimeHandler {
	start_time: Instant,
	time: Duration,
}

impl TimeHandler {
	pub fn new() -> Self {
		Self {
			start_time: Instant::now(),
			time: Duration::ZERO,
		}
	}

	pub fn update(&mut self) -> Duration {
		let prev_time = self.time;
		self.time = self.start_time.elapsed();
		self.time - prev_time
	}
}