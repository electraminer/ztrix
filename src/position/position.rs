use crate::serialize::FromChars;
use crate::serialize;
use crate::position::Vector;

use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;

use std::fmt;

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

impl fmt::Display for Position {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let x = if self.x < 0 {
			write!(f, "-")?;
			-(self.x + 1) as usize
		} else {
			self.x as usize
		};
		serialize::write_b64_var(f, x)?;
		let y = if self.y < 0 {
			write!(f, "-")?;
			-(self.y + 1) as usize
		} else {
			self.y as usize
		};
		serialize::write_b64_var(f, y)
	}
}

impl FromChars for Position {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		Ok(Position {
			x: match chars.peek().ok_or(())? {
				'-' => {
					chars.next();
					let i: i32 =
						serialize::read_b64_var(&mut chars)?
						.try_into().map_err(|_| ())?;
					-i - 1
				},
				_ => serialize::read_b64_var(&mut chars)?
						.try_into().map_err(|_| ())?,
			},	
			y: match chars.peek().ok_or(())? {
				'-' => {
					chars.next();
					let i: i32 =
						serialize::read_b64_var(&mut chars)?
						.try_into().map_err(|_| ())?;
					-i - 1
				},
				_ => serialize::read_b64_var(&mut chars)?
						.try_into().map_err(|_| ())?,
			}
		})
	}
}