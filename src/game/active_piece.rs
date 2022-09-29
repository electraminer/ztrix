use crate::game::Board;
use crate::game::Mino;
use crate::game::PieceType;
use crate::position::Rotation;
use crate::position::Position;
use crate::position::Vector;

#[derive(Debug, Clone)]
pub struct ActivePiece {
	pub piece_type: PieceType,
	pub pos: Position,
	pub rot: Rotation,
}

impl ActivePiece {
	pub fn spawn_unchecked(piece_type: PieceType, irs: Rotation)
			-> ActivePiece {
		let offset = piece_type.get_kicks(Rotation::Zero, irs)[0];
		let min_y = piece_type.get_mino_vecs().iter()
			.map(|v| v.rotate(irs).y + offset.y).min().unwrap_or(0);
		ActivePiece{
			piece_type: piece_type,
			pos: Position::new(4, 19 - min_y) + offset,
			rot: irs,
		}
	}

	pub fn spawn(board: &Board, piece_type: PieceType,
			irs: Rotation) -> Option<ActivePiece> {
		let piece = ActivePiece::spawn_unchecked(
				piece_type, irs);
		if !piece.is_colliding(board) {
			return Some(piece)
		}
		let piece = ActivePiece::spawn_unchecked(
				piece_type, Rotation::Zero);
		if !piece.is_colliding(board) {
			return Some(piece)
		}
		None
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
		let kicks = self.piece_type.get_kicks(self.rot, rot);
		self.rot = self.rot + rot;
		for kick in kicks {
			if self.try_move(board, kick) {
				return true;
			}
		}
		self.rot = self.rot - rot;
		return false;
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
