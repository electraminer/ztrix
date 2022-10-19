use crate::serialize::FromChars;
use std::str::FromStr;
use crate::game::MaybeActive;

use crate::replay::Info;
use crate::position::Rotation;
use crate::position::Vector;
use crate::game::BagRandomizer;
use crate::game::Board;

use crate::game::ActivePiece;
use crate::serialize;

use crate::game::Queue;

use crate::game::PieceType;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Action {
	MoveLeft,
	MoveRight,
	MoveDown,
	Rotate(Rotation),
	SpawnPiece(Rotation, bool),
	PlacePiece,
	HoldPiece(Rotation),
	ToggleZone,
	Init,
}

#[derive(PartialEq, Clone)]
pub enum Clear {
	LineClear(usize),
	ZoneClear(usize),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Game {
	pub piece: Option<MaybeActive>,
	pub queue: Queue,
	pub hold: Option<PieceType>,
	pub has_held: bool,
	pub board: Board,
	pub in_zone: bool,
	pub over: bool,
}

impl Game {
	pub fn init(&mut self, info: &mut Info) {
		self.queue.update(info);
		if let None = self.piece {
			self.piece = Some(
				MaybeActive::Inactive(self.queue.next(info)));
		}
	}

	pub fn get_current(&self) -> Option<PieceType> {
		self.piece.as_ref().and_then(|p| Some(p.get_type()))
	}

	pub fn get_active(&self) -> Option<&ActivePiece> {
		match &self.piece {
			Some(MaybeActive::Active(p)) => Some(&p),
			_ => None,
		}
	}

	fn move_piece(&mut self, vec: Vector) {
		if let Some(MaybeActive::Active(active)) = &mut self.piece {
			active.try_move(&self.board, vec);
		}
	}

	fn rotate_piece(&mut self, rot: Rotation) {
		if let Some(MaybeActive::Active(active)) = &mut self.piece {
			active.try_rotate(&self.board, rot);
		}
	}

	fn spawn(&mut self, irs: Rotation) -> Vec<Clear> {
		let mut clears = Vec::new();
		if let Some(MaybeActive::Inactive(current)) = self.piece {
			match ActivePiece::spawn(
				&self.board, current, irs).or_else(|| {
					if self.in_zone {
						clears.append(&mut self.toggle_in_zone())
					}
					ActivePiece::spawn(
						&self.board, current, irs)
				}) {
				Some(a) => self.piece = Some(MaybeActive::Active(a)),
				None => self.over = true,
			}
		}
		clears
	}

	fn hold(&mut self, irs: Rotation,
			info: &mut Info) -> Vec<Clear> {
		if self.has_held {
			return vec![];
		}
		if let Some(current) = self.get_current() {
			self.has_held = true;
			let swap = self.hold.unwrap_or_else(|| {
				self.queue.next(info)
			});
			self.hold = Some(current);
			self.piece = Some(MaybeActive::Inactive(swap));
			self.spawn(irs)
		} else {
			vec![]
		}
	}

	fn hold_if_active(&mut self, irs: Rotation,
			info: &mut Info) -> Vec<Clear> {
		if let Some(MaybeActive::Active(_)) = self.piece {
			self.hold(irs, info)
		} else {
			vec![]
		}
	}

	fn spawn_ihs(&mut self, irs: Rotation, ihs: bool,
			info: &mut Info) -> Vec<Clear> {
		let mut clears = Vec::new();
		if !(self.has_held && ihs) {
			clears.append(&mut self.spawn(irs));
		}
		if self.over {
			return clears;
		}
		self.has_held = false;
		if ihs {
			clears.append(&mut self.hold(irs, info));
		}
		clears
	}

	fn place(&mut self, info: &mut Info) -> Vec<Clear> {
		if let Some(MaybeActive::Inactive(_)) = self.piece {
			return vec![];
		}
		let current = self.queue.next(info);
		let piece = MaybeActive::Inactive(current);
		let active = match std::mem::replace(
				&mut self.piece, Some(piece)) {
			Some(MaybeActive::Active(a)) => a,
			_ => return vec![],
		};
		let height = active.get_ghost(&self.board)
			.get_mino_positions()
			.iter().map(|p| p.y).min().unwrap_or(0);
		active.place(&mut self.board);
		let cleared = if self.in_zone {
			self.board.clear_lines_zone()
		} else {
			self.board.clear_lines()
		};
		let mut clears = match cleared {
			0 => vec![],
			c => vec![Clear::LineClear(c)],
		};
		if height >= 20 {
			if self.in_zone {
				clears.append(&mut self.toggle_in_zone())
			} else {
				self.over = true;
			}
		}
		clears
	}

	fn toggle_in_zone(&mut self) -> Vec<Clear> {
		self.in_zone = !self.in_zone;
		if !self.in_zone {
			vec![Clear::ZoneClear(
				self.board.clear_lines())]
		} else {
			vec![]
		}
	}

	pub fn update(&mut self, action: Action,
			info: &mut Info) -> Vec<Clear> {
		if self.over {
			return vec![];
		}
		match action {
			Action::MoveLeft => {
				self.move_piece(Vector::ONE_LEFT); vec![]},
			Action::MoveRight => {
				self.move_piece(Vector::ONE_RIGHT); vec![]}
			Action::MoveDown => {
				self.move_piece(Vector::ONE_DOWN); vec![]},
			Action::Rotate(rot) => {
				self.rotate_piece(rot); vec![]}
			Action::SpawnPiece(irs, ihs) =>
				self.spawn_ihs(irs, ihs, info),
			Action::PlacePiece => self.place(info),
			Action::HoldPiece(irs) =>
				self.hold_if_active(irs, info),
			Action::ToggleZone => self.toggle_in_zone(),
			Action::Init => {self.init(info); vec![]},
		}
	}
}

impl Default for Game {
	fn default() -> Game {
		Game{
			piece: None,
			queue: Queue::new(
				BagRandomizer::new(), 4),
			hold: None,
			has_held: false,
			board: Board::new(),
			in_zone: false,
			over: false,
		}
	}
}

impl fmt::Display for Game {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		serialize::write_option(f, self.piece.as_ref())?;
		write!(f, "{}", self.queue)?;
		serialize::write_option(f, self.hold)?;
		serialize::write_bool(f, self.has_held)?;
		write!(f, "{}", self.board)?;
		serialize::write_bool(f, self.in_zone)?;
		serialize::write_bool(f, self.over)
	}
}

impl FromChars for Game {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		Ok(Game {
			piece: Option::from_chars(&mut chars)?,
			queue: Queue::from_chars(&mut chars)?,
			hold: Option::from_chars(&mut chars)?,
			has_held: bool::from_chars(&mut chars)?,
			board: Board::from_chars(&mut chars)?,
			in_zone: bool::from_chars(&mut chars)?,
			over: bool::from_chars(&mut chars)?,
		})
	}
}

impl FromStr for Game {
	type Err = ();
	fn from_str(string: &str) -> Result<Self, ()> {
		Self::from_chars(&mut string.chars())
	}
}