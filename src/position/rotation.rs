use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub enum Rotation {
	Zero,
	Clockwise,
	Flip,
	Anticlockwise,
}

impl Rotation {
	pub fn from_num_cw(num_cw: i32) -> Rotation {
		match num_cw % 4 {
			1 => Rotation::Clockwise,
			2 => Rotation::Flip,
			3 => Rotation::Anticlockwise,
			_ => Rotation::Zero,
		}
	}

	pub fn from_num_ccw(num_ccw: i32) -> Rotation {
		Self::from_num_cw(-num_ccw)
	}

	pub fn num_cw(self) -> i32 {
		match self {
			Rotation::Zero => 0,
			Rotation::Clockwise => 1,
			Rotation::Flip => 2,
			Rotation::Anticlockwise => 3,
		}
	}

	pub fn num_ccw(self) -> i32 {
		self.neg().num_cw()
	}
}

impl Add for Rotation {
	type Output = Rotation;
	fn add(self, rot: Rotation) -> Rotation {
		Rotation::from_num_cw(self.num_cw() + rot.num_cw())
	}
}

impl Neg for Rotation {
	type Output = Rotation;
	fn neg(self) -> Rotation {
		Rotation::from_num_ccw(self.num_cw())
	}
}

impl Sub for Rotation {
	type Output = Rotation;
	fn sub(self, rot: Rotation) -> Rotation {
		self + rot.neg()
	}
}