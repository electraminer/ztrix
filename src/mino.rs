use crate::piece::PieceType;

#[derive(Debug, Copy, Clone)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}