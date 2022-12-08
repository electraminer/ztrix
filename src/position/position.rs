use crate::serialize::SerializeUrlSafe;
use crate::position::Vector;

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

impl Position {
	pub const ORIGIN: Position = Position{x: 0, y: 0};

	pub fn new(x: i32, y: i32) -> Position {
		Position{x: x, y: y}
	}

}

impl Add<Vector> for Position {
	type Output = Position;
	fn add(self, vec: Vector) -> Position {
		Position::new(self.x + vec.x, self.y + vec.y)
	}
}

impl Sub<Vector> for Position {
	type Output = Position;
	fn sub(self, vec: Vector) -> Position {
		self + vec.neg()
	}
}

impl SerializeUrlSafe for Position {
	fn serialize(&self) -> String {
		format! { "{}{}",
			self.x.serialize(),
			self.y.serialize(),
		}
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(Self {
			x: i32::deserialize(input)?,
			y: i32::deserialize(input)?,
		})
	}
}