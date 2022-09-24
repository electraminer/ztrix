use crate::position::Rotation;

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vector {
	pub x: i32,
	pub y: i32,
}

impl Vector {
	pub const ZERO: Vector = Vector{x: 0, y: 0};
	pub const ONE_RIGHT: Vector = Vector{x: 1, y: 0};
	pub const ONE_LEFT: Vector = Vector{x: -1, y: 0};
	pub const ONE_UP: Vector = Vector{x: 0, y: 1};
	pub const ONE_DOWN: Vector = Vector{x: 0, y: -1};

	pub fn new(x: i32, y: i32) -> Vector {
		Vector{x: x, y: y}
	}

	pub fn rotate(self, rot: Rotation) -> Vector {
		match rot {
			Rotation::Zero => self,
			Rotation::Clockwise => Vector::new(self.y, -self.x),
			Rotation::Flip => -self,
			Rotation::Anticlockwise => -Vector::new(self.y, -self.x),
		}
	}
}

impl Add for Vector {
	type Output = Vector;
	fn add(self, vec: Vector) -> Vector {
		Vector::new(self.x + vec.x, self.y + vec.y)
	}
}

impl Neg for Vector {
	type Output = Vector;
	fn neg(self) -> Vector {
		Vector{x: -self.x, y: -self.y}
	}
}

impl Sub for Vector {
	type Output = Vector;
	fn sub(self, vec: Vector) -> Vector {
		self + vec.neg()
	}
}