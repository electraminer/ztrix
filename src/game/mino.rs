use crate::game::PieceType;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}