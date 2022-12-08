use std::fmt;

use std::iter::Peekable;
use std::str::Chars;


pub struct DeserializeError {
	msg: String,
}

impl DeserializeError {
	pub fn new<S>(msg: S) -> Self
	where S: Into<String> {
		Self {
			msg: msg.into()
		}
	}
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub struct DeserializeInput<'a> {
	iter: Peekable<Chars<'a>>
}

impl<'a> DeserializeInput<'a> {
	pub fn from(string: &'a String) -> Self {
		Self {
			iter: string.chars().peekable()
		}
	}

    pub fn next(&mut self) -> Result<char, DeserializeError> {
        self.iter.next().ok_or(DeserializeError::new("Ran out of characters"))
    }

    pub fn peek(&mut self) -> Result<char, DeserializeError> {
        self.iter.peek().copied().ok_or(DeserializeError::new("Ran out of characters"))
    }

	pub fn next_if(&mut self, c: char) -> Result<bool, DeserializeError> {
		let matched = self.peek()? == c;
		if matched {
			self.next()?;
		}
		Ok(matched)
	}
}

pub trait SerializeUrlSafe
where	Self: Sized {
	fn serialize(&self) -> String;
	
	fn serialize_with_escape(&self, escape: char, escaped_chars: &'static str) -> String {
		let serialized = self.serialize();
		let mut escaped_chars: Vec<char> = escaped_chars.chars().collect();
		escaped_chars.push(escape);
		if serialized.starts_with(escaped_chars.as_slice()) {
			format! {"{}{}", escape, serialized}
		} else {
			serialized
		}
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError>;

	fn deserialize_string<S>(string: S) -> Result<Self, DeserializeError>
	where 	S: Into<String> {
		Self::deserialize(&mut DeserializeInput::from(&string.into()))
	}

	fn serialize_array<const L: usize>(array: &[Self; L]) -> String {
		array.iter().map(|s| s.serialize()).collect()
	}

	fn deserialize_array<const L: usize>(input: &mut DeserializeInput) -> Result<[Self; L], DeserializeError> {
		[(); L].try_map(|_| Self::deserialize(input))
	}
}

impl<T, const L: usize> SerializeUrlSafe for [T; L]
where	T: SerializeUrlSafe {
	fn serialize(&self) -> String {
		T::serialize_array(self)
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		T::deserialize_array(input)
	}
}

impl<T> SerializeUrlSafe for Option<T>
where	T: SerializeUrlSafe {
	fn serialize(&self) -> String {
		match self {
			None => "_".to_owned(),
			Some(t) => t.serialize_with_escape('~', "_"),
		}
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		if input.next_if('_')? {
			return Ok(None);
		}
		input.next_if('~')?;
		Ok(Some(T::deserialize(input)?))
	}
}

impl<T> SerializeUrlSafe for Vec<T>
where	T: SerializeUrlSafe {
	fn serialize(&self) -> String {
		let mut string = String::new();
		for t in self.into_iter() {
			string.push_str(&t.serialize_with_escape('~', "."));
		}
		string + "."
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		let mut vec = Vec::new();
		while !input.next_if('.')? {
			input.next_if('~')?;
			vec.push(T::deserialize(input)?);
		}
		Ok(vec)
	}
}

const BASE64_CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-";

impl SerializeUrlSafe for bool {
	fn serialize(&self) -> String {
		match self {
			true => "T",
			false => "F",
		}.to_owned()
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		Ok(match input.next()? {
			'F' => false,
			'T' => true,
			_ => return Err(DeserializeError::new("Boolean should be represented by T or F.")),
		})
	}

	fn serialize_array<const L: usize>(array: &[Self; L]) -> String {
		array.iter()
			.chain(std::iter::repeat(&false))
			.array_chunks::<6>()
			.map(|chunk| chunk.iter()
				.rev()
				.fold(0, |acc, b| match b {
					true => acc * 2 + 1,
					false => acc * 2,
				})
			).map(|b64| BASE64_CHARSET.chars().nth(b64).expect("Should always be within range of 64 characters."))
			.take((L+5) / 6)
			.collect()
	}

	fn deserialize_array<const L: usize>(input: &mut DeserializeInput) -> Result<[Self; L], DeserializeError> {
		let mut out = [false; L];
		for chunk in 0..(L+5)/6 {
			let mut b64 = BASE64_CHARSET.find(input.next()?)
				.ok_or(DeserializeError::new("Base64 should consist of 0-9, A-Z, a-z, _, and -."))?;
			for i in 0..(L - chunk * 6) {
				if b64 % 2 == 1 {
					out[chunk * 6 + i] = true;
				}
				b64 /= 2;
			}
		}
		Ok(out)
	}
}

impl SerializeUrlSafe for usize {
	fn serialize(&self) -> String {
		let mut bin = *self;
		let mut string = String::new();
		while bin > 0 {
			let b64 = bin % 64;
			bin /= 64;
			string.push(BASE64_CHARSET.chars().nth(b64).expect("Should always be within range of 64 characters."));
		}
		string + "."
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		let mut bin = 0;
		let mut place_value: Self = 1;
		while !input.next_if('.')? {
			place_value.checked_mul(64)
				.ok_or(DeserializeError::new("Base64 was too large to fit in an integer."))?;
			bin += place_value * BASE64_CHARSET.find(input.next()?)
				.ok_or(DeserializeError::new("Base64 should consist of 0-9, A-Z, a-z, _, and -."))?;
			place_value *= 64;
		}
		Ok(bin)
	}
}

impl SerializeUrlSafe for isize {
	fn serialize(&self) -> String {
		if *self < 0 {
			"~".to_owned() + &(-*self as usize).serialize()
		} else {
			(*self as usize).serialize()
		}
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		if input.next_if('~')? {
			Ok((0isize).checked_sub_unsigned(usize::deserialize(input)?)
				.ok_or(DeserializeError::new("Base64 was too large to fit in an integer."))?)
		} else {
			Ok(usize::deserialize(input)?.try_into()
				.map_err(|_| DeserializeError::new("Base64 was too large to fit in an integer."))?)
		}
	}
}

impl SerializeUrlSafe for i32 {
	fn serialize(&self) -> String {
		(*self as isize).serialize()
	}

	fn deserialize(input: &mut DeserializeInput) -> Result<Self, DeserializeError> {
		Ok(isize::deserialize(input)?.try_into()
			.map_err(|_| DeserializeError::new("Base64 was too large to fit in an integer."))?)
	}
}