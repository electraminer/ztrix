use crate::serialize::FromChars;
use std::str::FromStr;
use crate::game::MaybeActive;

use crate::replay::Info;
use crate::position::Rotation;
use crate::position::Vector;
use crate::game::BagRandomizer;
use crate::game::Board;

use crate::game::ActivePiece;

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
	PlacePiece(Rotation, bool),
	HoldPiece(Rotation),
	ToggleZone,
}

pub enum Clear {
	LineClear(usize),
	ZoneClear(usize),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Game {
	pub piece: MaybeActive,
	pub queue: Queue,
	pub hold: Option<PieceType>,
	pub has_held: bool,
	pub board: Board,
	pub in_zone: bool,
	pub over: bool,
}

impl Game {
	pub fn new() -> Game {
		let mut info = Info::new();
		let mut rando = BagRandomizer::new();
		let piece = rando.next(&mut info);
		Game{
			piece: MaybeActive::Inactive(piece),
			queue: Queue::new(rando, &mut info),
			hold: None,
			has_held: false,
			board: Board::new(),
			in_zone: false,
			over: false,
		}
	}

	pub fn get_current(&self) -> PieceType {
		self.piece.get_type()
	}

	pub fn get_active(&self) -> Option<&ActivePiece> {
		match &self.piece {
			MaybeActive::Active(p) => Some(p),
			MaybeActive::Inactive(_) => None,
		}
	}

	fn move_piece(&mut self, vec: Vector) {
		if let MaybeActive::Active(active) = &mut self.piece {
			active.try_move(&self.board, vec);
		}
	}

	fn rotate_piece(&mut self, rot: Rotation) {
		if let MaybeActive::Active(active) = &mut self.piece {
			active.try_rotate(&self.board, rot);
		}
	}

	fn hold(&mut self, info: &mut Info) {
		if self.has_held {
			return;
		}
		self.has_held = true;
		let swap = self.hold.unwrap_or_else(|| {
			self.queue.next(info)
		});
		self.hold = Some(self.get_current());
		self.piece = MaybeActive::Inactive(swap);
	}

	fn spawn(&mut self, irs: Rotation, ihs: bool,
			info: &mut Info) -> Vec<Clear> {
		if !ihs {
			self.hold(info);
		}
		let mut clears = Vec::new();
		if let MaybeActive::Inactive(current) = self.piece {
			match ActivePiece::spawn(
				&self.board, current, irs).or_else(|| {
					if self.in_zone {
						clears.append(&mut self.toggle_in_zone())
					}
					ActivePiece::spawn(
						&self.board, current, irs)
				}) {
				Some(a) => self.piece = MaybeActive::Active(a),
				None => self.over = true,
			}
		}
		clears
	}

	fn place(&mut self, irs: Rotation, ihs: bool,
			info: &mut Info) -> Vec<Clear> {
		if let MaybeActive::Inactive(_) = self.piece {
			return vec![];
		}
		let current = self.queue.next(info);
		let piece = MaybeActive::Inactive(current);
		let active = match std::mem::replace(
				&mut self.piece, piece) {
			MaybeActive::Active(a) => a,
			MaybeActive::Inactive(_) => return vec![],
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
		self.has_held = false;
		if ihs {
			self.hold(info);
		}
		if matches! { ActivePiece::spawn(
			&self.board, self.get_current(), irs), None }
			|| height >= 20 {
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
				self.spawn(irs, ihs, info),
			Action::PlacePiece(irs, ihs) =>
				self.place(irs, ihs, info),
			Action::HoldPiece(irs) =>
				self.spawn(irs, true, info),
			Action::ToggleZone => self.toggle_in_zone(),
		}
	}
}

impl Default for Game {
	fn default() -> Game {
		Game::new()
	}
}

impl fmt::Display for Game {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.piece)?;
		write!(f, "{}", self.queue)?;
		match self.hold {
			None => write!(f, "_")?,
			Some(piece) => write!(f, "{}", piece)?,
		}
		match self.has_held {
			false => write!(f, "F")?,
			true => write!(f, "T")?,
		}
		write!(f, "{}", self.board)?;
		match self.in_zone {
			false => write!(f, "F")?,
			true => write!(f, "T")?,
		}
		match self.over {
			false => write!(f, "F"),
			true => write!(f, "T"),
		}
	}
}

impl FromChars for Game {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		let mut chars = chars.peekable();
		Ok(Game {
			piece: MaybeActive::from_chars(&mut chars)?,
			queue: Queue::from_chars(&mut chars)?,
			hold: match chars.peek().ok_or(())? {
				'_' => {
					chars.next();
					None
				},
				_ => Some(PieceType::from_chars(&mut chars)?),
			},
			has_held: chars.next().ok_or(())? == 'T',
			board: Board::from_chars(&mut chars)?,
			in_zone: chars.next().ok_or(())? == 'T',
			over: chars.next().ok_or(())? == 'T',
		})
	}
}

impl FromStr for Game {
	type Err = ();
	fn from_str(string: &str) -> Result<Self, ()> {
		Self::from_chars(&mut string.chars())
	}
}