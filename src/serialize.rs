use std::fmt;

pub trait FromChars {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized;
}

impl FromChars for bool {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		Ok(match chars.next().ok_or(())? {
			'F' => false,
			'T' => true,
			_ => return Err(()),
		})
	}
}

const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-";

pub fn write_b64_var(f: &mut fmt::Formatter, mut bin: usize)
		-> fmt::Result {
	while bin > 0 {
		let b64 = bin % 64;
		bin = bin / 64;
		write!(f, "{}", CHARSET.chars()
			.nth(b64 as usize).unwrap())?;
	}
	write!(f, ".")
}

pub fn write_b64_fixed(f: &mut fmt::Formatter, mut bin: usize,
		len: usize) -> fmt::Result {
	for _ in 0..len {
		let b64 = bin % 64;
		bin = bin / 64;
		write!(f, "{}", CHARSET.chars()
			.nth(b64 as usize).unwrap())?;
	}
	Ok(())
}

pub fn read_b64_var<I>(chars: &mut I) -> Result<usize, ()>
	where 	I: Iterator<Item = char> {
	let mut bin = 0;
	let chars: String = chars.take_while(|c| *c != '.')
		.collect();
	for c in chars.chars().rev() {
		bin *= 64;
		bin += CHARSET.find(c).ok_or(())?;
	}
	Ok(bin)
}

pub fn read_b64_fixed<I>(chars: &mut I,
		len: usize) -> Result<usize, ()>
	where 	I: Iterator<Item = char> {
	let mut bin = 0;
	let chars: String = chars.take(len).collect();
	if chars.len() != len {
		return Err(());
	}
	for c in chars.chars().rev() {
		bin *= 64;
		bin += CHARSET.find(c).ok_or(())?;
	}
	Ok(bin)
}