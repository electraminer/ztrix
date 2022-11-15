use crate::serialize::FromChars;
use crate::game::Board;
use crate::game::Mino;
use crate::game::PieceType;
use crate::position::Rotation;
use crate::position::Position;
use crate::position::Vector;

use std::fmt;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
		ActivePiece {
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
			rot: Rotation) -> Option<usize> {
		let kicks = self.piece_type.get_kicks(self.rot, rot);
		self.rot = self.rot + rot;
		for (i, kick) in kicks.into_iter().enumerate() {
			if self.try_move(board, kick) {
				return Some(i);
			}
		}
		self.rot = self.rot - rot;
		None
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

impl fmt::Display for ActivePiece {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.piece_type)?;
		write!(f, "{}", self.pos)?;
		write!(f, "{}", self.rot)
	}
}

impl FromChars for ActivePiece {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		Ok(ActivePiece {
			piece_type: PieceType::from_chars(chars)?,
			pos: Position::from_chars(chars)?,
			rot: Rotation::from_chars(chars)?,
		})
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum MaybeActive {
	Active(ActivePiece),
	Inactive(PieceType),
}

impl MaybeActive {
	pub fn get_type(&self) -> PieceType {
		match self {
			MaybeActive::Active(p) => p.get_type(),
			MaybeActive::Inactive(p) => *p,
		}
	}
}

impl fmt::Display for MaybeActive {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MaybeActive::Active(p) => write!(f, "A{}", p),
			MaybeActive::Inactive(p) => write!(f, "I{}", p),
		}
	}
}

impl FromChars for MaybeActive {
	fn from_chars<I>(chars: &mut I) -> Result<Self, ()>
	where 	I: Iterator<Item = char>,
			Self: Sized {
		Ok(match chars.next().ok_or(())? {
			'A' => MaybeActive::Active(
				ActivePiece::from_chars(chars)?),
			'I' => MaybeActive::Inactive(
				PieceType::from_chars(chars)?),
			_ => return Err(())
		})
	}
}