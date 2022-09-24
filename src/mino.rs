use crate::piece::PieceType;

#[derive(Copy, Clone)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}