//use crate::serialize::FromChars;
use std::str::FromStr;

use crate::position::Rotation;
use crate::position::Vector;
use crate::game::BagRandomizer;
use crate::game::Board;

use crate::game::ActivePiece;
use crate::serialize;

use crate::game::Queue;

use crate::game::PieceType;
use crate::serialize::DeserializeError;
use crate::serialize::SerializeUrlSafe;

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
}

pub enum RevealAction {
	Reveal(usize),
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
	pub piece: Option<ActivePiece>,
	pub queue: Queue,
	pub hold: Option<PieceType>,
	pub has_held: bool,
	pub board: Board,
	pub in_zone: bool,
	pub over: bool,
}

impl Game {
	fn move_piece<F>(&mut self, vec: Vector, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(piece) = &mut self.piece {
			if piece.try_move(&self.board, vec) {
				event_handler(&Event::Move);
			}
		}
	}

	fn rotate_piece<F>(&mut self, rot: Rotation, event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(piece) = &mut self.piece {
			if let Some(kick) = piece.try_rotate(&self.board, rot) {
				event_handler(&Event::Rotate(kick));
			}
		}
	}

	fn create_piece<F>(&mut self, piece_type: PieceType, irs: Rotation, event_handler: &mut F)
			-> Option<ActivePiece>
	where	F: FnMut(&Event) {
		if let Some(piece) = ActivePiece::spawn(&self.board, piece_type, irs) {
			return Some(piece);
		}
		if let Some(piece) = ActivePiece::spawn(&self.board, piece_type, Rotation::Zero) {
			return Some(piece);
		}

		if self.in_zone {
			self.toggle_in_zone(event_handler);
		}

		if let Some(piece) = ActivePiece::spawn(&self.board, piece_type, irs) {
			return Some(piece);
		}
		if let Some(piece) = ActivePiece::spawn(&self.board, piece_type, Rotation::Zero) {
			return Some(piece);
		}

		None
	}

	fn spawn<F>(&mut self, piece_type: PieceType, irs: Rotation, event_handler: &mut F)
	where	F: FnMut(&Event) {
		self.piece = self.create_piece(piece_type, irs, event_handler);
		match self.piece {
			Some(_) => event_handler(&Event::Spawn),
			None => self.over = true,
		}
	}

	fn hold<F>(&mut self, irs: Rotation, event_handler: &mut F) -> Result<(), RevealAction>
	where	F: FnMut(&Event) {
		if self.has_held {
			return Ok(());
		}

		match self.piece {
			Some(piece) => piece.get_type(),
			None => self.queue.next().map_err(|seed| RevealAction::Reveal(seed))?,
		};

		let piece_to_hold = match self.get_current() {
			None => {
				let piece = self.queue.next()
				self.piece = MaybeActive::Inactive(Some(piece));
				piece
			}
			Some(piece) => piece,
		};


		let piece_to_swap_to = self.hold.or_else(|| self.queue.next().ok());

		self.has_held = true;
		self.hold = Some(piece_to_hold);
		self.piece = MaybeActive::Inactive(piece_to_swap_to);
		self.spawn(irs, event_handler);
		Ok(())
	}

	fn hold_if_active<F>(&mut self, irs: Rotation,
			event_handler: &mut F)
	where	F: FnMut(&Event) {
		if let Some(MaybeActive::Active(_)) = self.piece {
			self.hold(irs, event_handler);
		}
	}

	fn spawn_ihs<F>(&mut self, irs: Rotation, ihs: bool,
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
			self.hold(irs, event_handler);
		}
	}

	fn place<F>(&mut self, event_handler: &mut F)
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

	pub fn update<F>(&mut self, action: Action, event_handler: &mut F) -> Result<(), RevealAction>
	where	F: FnMut(&Event) {
		if self.over {
			return Ok(());
		}
		match action {
			Action::MoveLeft => self.move_piece(Vector::ONE_LEFT, event_handler),
			Action::MoveRight => self.move_piece(Vector::ONE_RIGHT, event_handler),
			Action::MoveDown => self.move_piece(Vector::ONE_DOWN, event_handler),
			Action::Rotate(rot) => self.rotate_piece(rot, event_handler),
			Action::SpawnPiece(irs, ihs) => self.spawn_ihs(irs, ihs, event_handler),
			Action::PlacePiece => self.place(event_handler),
			Action::HoldPiece(irs) => self.hold_if_active(irs, event_handler),
			Action::ToggleZone => self.toggle_in_zone(event_handler),
		}
		Ok(())
	}

	pub fn reveal(&mut self, action: RevealAction) {

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

impl SerializeUrlSafe for Game {
	fn serialize(&self) -> String {
		format! { "{}{}{}{}{}{}{}",
			self.piece.serialize(),
			self.queue.serialize(),
			self.hold.serialize(),
			self.has_held.serialize(),
			self.board.serialize(),
			self.in_zone.serialize(),
			self.over.serialize(),
		}
	}

	fn deserialize(input: &mut serialize::DeserializeInput) -> Result<Self, serialize::DeserializeError> {
		Ok(Game {
			piece: Option::deserialize(input)?,
			queue: Queue::deserialize(input)?,
			hold: Option::deserialize(input)?,
			has_held: bool::deserialize(input)?,
			board: Board::deserialize(input)?,
			in_zone: bool::deserialize(input)?,
			over: bool::deserialize(input)?,
		})
	}
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl FromStr for Game {
	type Err = DeserializeError;
	fn from_str(string: &str) -> Result<Self, DeserializeError> {
		Self::deserialize_string(string)
	}
}