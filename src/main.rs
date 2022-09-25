use ztrix::position::Rotation;
use ztrix::piece::PieceType;

fn main() {
	for kick in PieceType::J.get_kicks(
			Rotation::Zero, Rotation::Clockwise) {
		println!("{} {}", kick.x, kick.y);
	}
}