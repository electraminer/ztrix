use crate::board::Board;
use crate::mino::Mino;
use crate::piece::PieceType;
use crate::position::Rotation;
use crate::position::Position;
use crate::position::Vector;

#[derive(Clone)]
pub struct ActivePiece {
	piece_type: PieceType,
	pos: Position,
	rot: Rotation,
}

impl ActivePiece {
	pub fn spawn(piece_type: PieceType, irs: Rotation)
			-> ActivePiece {
		let min_y = piece_type.get_mino_vecs().iter()
			.map(|v| v.rotate(irs).y).min().unwrap_or(0);
		ActivePiece{
			piece_type: piece_type,
			pos: Position::new(4, 19 - min_y),
			rot: irs,
		}
	}

	pub fn get_type(&self) -> PieceType {
		self.piece_type
	}

	pub fn get_mino_positions(&self) -> [Position; 4] {
		self.piece_type.get_mino_vecs().map(|v| {
			self.pos + v.rotate(self.rot)})
	}

	pub fn is_colliding(&self, board: &Board) -> bool {
		self.get_mino_positions().iter().any(|&p| {
			matches!(board[p], Some(_))
		})
	}

	pub fn try_move(&mut self, board: &Board,
			vec: Vector) -> bool {
		self.pos = self.pos + vec;
		if self.is_colliding(board) {
			self.pos = self.pos - vec;
			return false;
		}
		return true;
	}

	pub fn try_rotate(&mut self, board: &Board,
			rot: Rotation) -> bool {
		self.rot = self.rot + rot;
		if self.is_colliding(board) {
			self.rot = self.rot - rot;
			return false;
		}
		return true;
	}

	pub fn get_ghost(&self, board: &Board) -> ActivePiece {
		let mut ghost = self.clone();
		while ghost.try_move(board, Vector::ONE_DOWN) {
			// ghost is moving
		}
		ghost
	}

	pub fn place(self, board: &mut Board) {
		for pos in self.get_ghost(board).get_mino_positions() {
			board[pos] = Some(Mino::Piece(self.piece_type));
		}
	}
}
