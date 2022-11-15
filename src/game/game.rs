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

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct LineClear {
	pub lines: usize,
	pub active: ActivePiece,
	pub board: Board,
	pub in_zone: bool,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Event {
	LineClear(LineClear),
	ZoneClear(usize),
	Spawn,
	Move,
	Rotate(usize),
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

	fn move_piece<F>(&mut self, vec: Vector, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Active(active)) = &mut self.piece {
			if active.try_move(&self.board, vec) {
				event_handler(&Event::Move);
			}
		}
	}

	fn rotate_piece<F>(&mut self, rot: Rotation, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Active(active)) = &mut self.piece {
			if let Some(kick) = active.try_rotate(&self.board, rot) {
				event_handler(&Event::Rotate(kick));
			}
		}
	}

	fn spawn<F>(&mut self, irs: Rotation, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Inactive(current)) = self.piece {
			match ActivePiece::spawn(
				&self.board, current, irs).or_else(|| {
					if self.in_zone {
						self.toggle_in_zone(event_handler);
					}
					ActivePiece::spawn(
						&self.board, current, irs)
				}) {
				Some(a) => self.piece = Some(MaybeActive::Active(a)),
				None => self.over = true,
			}
		}
		event_handler(&Event::Spawn);
	}

	fn hold<F>(&mut self, irs: Rotation, info: &mut Info, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if self.has_held {
			return;
		}
		if let Some(current) = self.get_current() {
			self.has_held = true;
			let swap = self.hold.unwrap_or_else(|| {
				self.queue.next(info)
			});
			self.hold = Some(current);
			self.piece = Some(MaybeActive::Inactive(swap));
			self.spawn(irs, event_handler);
		}
	}

	fn hold_if_active<F>(&mut self, irs: Rotation, info: &mut Info,
			event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Active(_)) = self.piece {
			self.hold(irs, info, event_handler);
		}
	}

	fn spawn_ihs<F>(&mut self, irs: Rotation, ihs: bool, info: &mut Info,
			event_handler: &mut F)
	where	F: FnMut(&Event) {
		if !(self.has_held && ihs) {
			self.spawn(irs, event_handler);
		}
		if self.over {
			return;
		}
		self.has_held = false;
		if ihs {
			self.hold(irs, info, event_handler);
		}
	}

	fn place<F>(&mut self, info: &mut Info, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Inactive(_)) = self.piece {
			return;
		}
		let current = self.queue.next(info);
		let piece = MaybeActive::Inactive(current);
		let active = match std::mem::replace(
				&mut self.piece, Some(piece)) {
			Some(MaybeActive::Active(a)) => a,
			_ => return,
		};
		let board = self.board.clone();
		let ghost = active.get_ghost(&self.board);
		let height = ghost.get_mino_positions().iter()
			.map(|p| p.y).min().unwrap_or(0);
		active.place(&mut self.board);
		let lines = if self.in_zone {
			self.board.clear_lines_zone()
		} else {
			self.board.clear_lines()
		};
		event_handler(&Event::LineClear(LineClear {
			lines, active: ghost, board, in_zone: self.in_zone }));
		if height >= 20 {
			if self.in_zone {
				self.toggle_in_zone(event_handler);
			} else {
				self.over = true;
			}
		}
	}

	fn toggle_in_zone<F>(&mut self, event_handler: &mut F)
	where	F: FnMut(&Event) {
		self.in_zone = !self.in_zone;
		if !self.in_zone {
			let lines = self.board.clear_lines();
			event_handler(&Event::ZoneClear(lines));
		}
	}

	pub fn update<F>(&mut self, action: Action, info: &mut Info,
			event_handler: &mut F)
	where	F: FnMut(&Event) {
		if self.over {
			return;
		}
		match action {
			Action::MoveLeft =>
				self.move_piece(Vector::ONE_LEFT, event_handler),
			Action::MoveRight =>
				self.move_piece(Vector::ONE_RIGHT, event_handler),
			Action::MoveDown =>
				self.move_piece(Vector::ONE_DOWN, event_handler),
			Action::Rotate(rot) => self.rotate_piece(rot, event_handler),
			Action::SpawnPiece(irs, ihs) =>
				self.spawn_ihs(irs, ihs, info, event_handler),
			Action::PlacePiece => self.place(info, event_handler),
			Action::HoldPiece(irs) =>
				self.hold_if_active(irs, info, event_handler),
			Action::ToggleZone => self.toggle_in_zone(event_handler),
			Action::Init => self.init(info),
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