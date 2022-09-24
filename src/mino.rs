use crate::piece::piece_type::PieceType;

#[derive(Copy, Clone)]
pub enum Mino {
	Piece(PieceType),
	Gray,
}