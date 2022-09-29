use crate::game::PieceType;

#[derive(Debug, Copy, Clone)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}