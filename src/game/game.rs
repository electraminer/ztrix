use crate::replay::Info;
use crate::position::Rotation;
use crate::position::Vector;
use crate::game::BagRandomizer;
use crate::game::Board;

use crate::game::ActivePiece;

use crate::game::Queue;

use crate::game::PieceType;

#[derive(Copy, Clone)]
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

#[derive(Clone)]
pub struct Game {
	current: PieceType,
	queue: Queue,
	hold: Option<PieceType>,
	has_held: bool,
	board: Board,
	piece: Option<ActivePiece>,
	in_zone: bool,
	over: bool,
}

impl Game {
	pub fn new() -> Game {
		let mut info = Info::new();
		let mut rando = BagRandomizer::new();
		Game{
			current: rando.next(&mut info),
			queue: Queue::new(rando, &mut info),
			hold: None,
			has_held: false,
			board: Board::new(),
			piece: None,
			in_zone: false,
			over: false,
		}
	}

	pub fn get_current(&self) -> PieceType {
		self.current
	}

	pub fn get_queue(&self) -> &Queue {
		&self.queue
	}

	pub fn get_hold(&self) -> Option<PieceType> {
		self.hold
	}

	pub fn has_held(&self) -> bool {
		self.has_held
	}

	pub fn get_board(&self) -> &Board {
		&self.board
	}

	pub fn get_piece(&self) -> &Option<ActivePiece> {
		&self.piece
	}

	pub fn is_in_zone(&self) -> bool {
		self.in_zone
	}

	pub fn is_over(&self) -> bool {
		self.over
	}

	fn move_piece(&mut self, vec: Vector) {
		if let Some(piece) = self.piece.as_mut() {
			piece.try_move(&self.board, vec);
		}
	}

	fn rotate_piece(&mut self, rot: Rotation) {
		if let Some(piece) = self.piece.as_mut() {
			piece.try_rotate(&self.board, rot);
		}
	}

	fn hold(&mut self, info: &mut Info) -> bool {
		if self.has_held {
			return false;
		}
		self.has_held = true;
		let mut used_rng = false;
		let swap = self.hold.unwrap_or_else(|| {
			used_rng = true;
			self.queue.next(info)
		});
		self.hold = Some(self.current);
		self.current = swap;
		used_rng
	}

	fn hold_spawn(&mut self, irs: Rotation,
			info: &mut Info) -> bool {
		let prev_held = self.has_held;
		let used_rng = self.hold(info);
		if !prev_held && self.has_held {
			self.piece = ActivePiece::spawn(
				&self.board, self.current, irs);
			if let None = self.piece {
				if self.in_zone {
					self.toggle_in_zone();
				}
				self.piece = ActivePiece::spawn(
					&self.board, self.current, irs);
				if let None = self.piece {
					self.over = true;
				}
			}
		}
		used_rng
	}

	fn spawn(&mut self, irs: Rotation, ihs: bool,
			info: &mut Info) -> bool {
		if let Some(_) = self.piece {
			return false;
		}
		let used_rng = if ihs {
			self.hold(info)
		} else {
			false
		};
		self.piece = ActivePiece::spawn(
			&self.board, self.current, irs);
		if let None = self.piece {
			if self.in_zone {
				self.toggle_in_zone();
			}
			self.piece = ActivePiece::spawn(
				&self.board, self.current, irs);
			if let None = self.piece {
				self.over = true;
			}
		}
		used_rng
	}

	fn place(&mut self, irs: Rotation, ihs: bool,
			info: &mut Info) -> bool {
		let piece = match std::mem::replace(
				&mut self.piece, None) {
			Some(p) => p,
			None => return false,
		};
		let height = piece.get_ghost(&self.board)
			.get_mino_positions()
			.iter().map(|p| p.y).min().unwrap_or(0);
		piece.place(&mut self.board);
		if self.in_zone {
			self.board.clear_lines_zone();
		} else {
			self.board.clear_lines();
		}
		self.current = self.queue.next(info);
		self.has_held = false;
		if ihs {
			self.hold(info);
		}
		if matches! { ActivePiece::spawn(
			&self.board, self.current, irs), None }
			|| height >= 20 {
			if self.in_zone {
				self.toggle_in_zone();
			} else {
				self.over = true;
			}
		}
		true
	}

	fn toggle_in_zone(&mut self) {
		if self.in_zone {
			self.board.clear_lines();
		}
		self.in_zone = !self.in_zone;
	}

	pub fn update(&mut self, action: Action,
			info: &mut Info) -> bool {
		if self.over {
			return false;
		}
		match action {
			Action::MoveLeft => {
				self.move_piece(Vector::ONE_LEFT); false},
			Action::MoveRight => {
				self.move_piece(Vector::ONE_RIGHT); false}
			Action::MoveDown => {
				self.move_piece(Vector::ONE_DOWN); false},
			Action::Rotate(rot) => {
				self.rotate_piece(rot); false}
			Action::SpawnPiece(irs, ihs) =>
				self.spawn(irs, ihs, info),
			Action::PlacePiece(irs, ihs) =>
				self.place(irs, ihs, info),
			Action::HoldPiece(irs) =>
				self.hold_spawn(irs, info),
			Action::ToggleZone => {
				self.toggle_in_zone(); false},
		}
	}
}