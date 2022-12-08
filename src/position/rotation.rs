use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Rotation {
	Zero,
	Clockwise,
	Flip,
	Anticlockwise,
}

impl Rotation {
	pub fn from_num_cw(num_cw: i32) -> Rotation {
		match num_cw.rem_euclid(4) {
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

impl SerializeUrlSafe for Rotation {
	fn serialize(&self) -> String {
		match self {
			Self::Zero => "0",
			Self::Clockwise => "C",
			Self::Flip => "2",
			Self::Anticlockwise => "A",
		}.to_owned()
	}

	fn deserialize(input: &mut crate::serialize::DeserializeInput) -> Result<Self, crate::serialize::DeserializeError> {
		Ok(match input.next()? {
			'0' => Self::Zero,
			'C' => Self::Clockwise,
			'2' => Self::Flip,
			'A' => Self::Anticlockwise,
			_ => return Err(DeserializeError::new("Rotation should be represented by 0, C, 2, or A.")),
		})
	}
}