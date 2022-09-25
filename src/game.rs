use crate::position::Rotation;
use crate::position::Vector;
use crate::randomizer::BagRandomizer;
use crate::board::Board;

use crate::piece::ActivePiece;

use crate::queue::Queue;

use crate::piece::PieceType;

pub enum Action {
	MoveLeft,
	MoveRight,
	MoveDown,
	Rotate(Rotation),
	SpawnPiece(Rotation, bool),
	PlacePiece(Rotation, bool),
	HoldPiece(Rotation),
}

pub struct Game {
	current: PieceType,
	queue: Queue,
	hold: Option<PieceType>,
	held: bool,
	board: Board,
	piece: Option<ActivePiece>,
	over: bool,
}

impl Game {
	pub fn new() -> Game {
		let mut rando = BagRandomizer::new(rand::thread_rng());
		Game{
			current: rando.next(),
			queue: Queue::new(rando),
			hold: None,
			held: false,
			board: Board::new(),
			piece: None,
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

	pub fn get_held(&self) -> bool {
		self.held
	}

	pub fn get_board(&self) -> &Board {
		&self.board
	}

	pub fn get_piece(&self) -> &Option<ActivePiece> {
		&self.piece
	}

	pub fn is_over(&self) -> bool {
		self.over
	}

	fn move_piece(&mut self, vec: Vector) -> bool {
		match self.piece.as_mut() {
			Some(piece) => piece.try_move(&self.board, vec),
			None => false,
		}
	}

	fn rotate_piece(&mut self, rot: Rotation) -> bool {
		match self.piece.as_mut() {
			Some(piece) => piece.try_rotate(&self.board, rot),
			None => false,
		}
	}

	fn hold(&mut self) -> bool {
		if self.held {
			return false;
		}
		self.held = true;
		let swap = self.hold.unwrap_or_else(||
			self.queue.next());
		self.hold = Some(self.current);
		self.current = swap;
		true
	}

	fn hold_spawn(&mut self, irs: Rotation) -> bool {
		let held = self.hold();
		if held {
			self.piece = ActivePiece::spawn(
				&self.board, self.current, irs);
			if let None = self.piece {
				self.over = true;
			}
		}
		held
	}

	fn spawn(&mut self, irs: Rotation, ihs: bool) -> bool {
		if let Some(_) = self.piece {
			return false;
		}
		if ihs {
			self.hold();
		}
		self.piece = ActivePiece::spawn(
			&self.board, self.current, irs);
		if let None = self.piece {
			self.over = true;
		}
		true
	}

	fn place(&mut self, irs: Rotation, ihs: bool) -> bool {
		let piece = match std::mem::replace(
				&mut self.piece, None) {
			Some(p) => p,
			None => return false,
		};
		let height = piece.get_ghost(&self.board)
			.get_mino_positions()
			.iter().map(|p| p.y).min().unwrap_or(0);
		piece.place(&mut self.board);
		self.board.clear_lines();
		self.current = self.queue.next();
		self.held = false;
		if ihs {
			self.hold();
		}
		if let None = ActivePiece::spawn(
			&self.board, self.current, irs) {
			self.over = true;
		}
		if height >= 20 {
			self.over = true;
		}
		true
	}

	pub fn execute(&mut self, action: Action) -> bool {
		if self.over {
			return false;
		}
		match action {
			Action::MoveLeft => self.move_piece(Vector::ONE_LEFT),
			Action::MoveRight => self.move_piece(Vector::ONE_RIGHT),
			Action::MoveDown => self.move_piece(Vector::ONE_DOWN),
			Action::Rotate(rot) => self.rotate_piece(rot),
			Action::SpawnPiece(irs, ihs) => self.spawn(irs, ihs),
			Action::PlacePiece(irs, ihs) => self.place(irs, ihs),
			Action::HoldPiece(irs) => self.hold_spawn(irs),
		}
	}
}