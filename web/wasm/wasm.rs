use ztrix::piece::active_piece::ActivePiece;
use ztrix::mino::Mino;
use ztrix::position::position::Position;
use ztrix::position::rotation::Rotation;
use web_sys::UrlSearchParams;
use wasm_bindgen::JsCast;
extern crate wasm_bindgen;
use web_sys::Element;
use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;


extern crate ztrix;
use ztrix::board::Board;
use ztrix::piece::piece_type::PieceType;

type JsResult<T> = Result<T, JsValue>;

trait RenderHtml<E>
where	E: Into<Element> {
	fn init_html(&self, document: &Document) -> JsResult<E>;

	fn update_html(&self, elem: &E) -> JsResult<()>;
}

fn get_piece_from_char(chr: char) -> Option<PieceType> {
	match chr {
		's' | 'S' => Some(PieceType::S),
		'z' | 'Z' => Some(PieceType::Z),
		'j' | 'J' => Some(PieceType::J),
		't' | 'T' => Some(PieceType::T),
		'l' | 'L' => Some(PieceType::L),
		'o' | 'O' => Some(PieceType::O),
		'i' | 'I' => Some(PieceType::I),
		_ => None,
	}
}

fn get_mino_from_char(chr: char) -> Option<Mino> {
	if let Some(piece) = get_piece_from_char(chr) {
		return Some(Mino::Piece(piece));
	}
	if matches!(chr, 'g' | 'G') {
		return Some(Mino::Gray);
	}
	return None
}

fn get_board_from_str(string: &str) -> Board {
	let mut board = Board::new();
	let mut iter = string.chars();
	for y in 0..26 {
		for x in 0..10 {
			let pos = Position::new(x, y);
			if let Some(chr) = iter.next() {
				board[pos] = get_mino_from_char(chr)
			}
		}
	}
	board
}

fn get_mino_css_color(mino: Mino) -> &'static str {
	match mino {
		Mino::Piece(PieceType::S) => "#1A1",
		Mino::Piece(PieceType::Z) => "#C12",
		Mino::Piece(PieceType::J) => "#03D",
		Mino::Piece(PieceType::T) => "#819",
		Mino::Piece(PieceType::L) => "#C51",
		Mino::Piece(PieceType::O) => "#CA0",
		Mino::Piece(PieceType::I) => "#29D",
		Mino::Gray => "#666",
	}
}

fn get_piece_css_color(piece: PieceType) -> &'static str {
	match piece {
		PieceType::S => "#080",
		PieceType::Z => "#900",
		PieceType::J => "#00B",
		PieceType::T => "#617",
		PieceType::L => "#A30",
		PieceType::O => "#A80",
		PieceType::I => "#07B",
	}
}

impl RenderHtml<HtmlCanvasElement> for Board {
	fn init_html(&self, document: &Document)
			-> JsResult<HtmlCanvasElement> {
		let elem = document.create_element("canvas")?
		.dyn_into::<HtmlCanvasElement>()?;
		elem.set_class_name("board");
    	Ok(elem)
	}

	fn update_html(&self, elem: &HtmlCanvasElement)
			-> JsResult<()> {
		// rescale component to match its size
		let width = elem.offset_width() as f64;
		let height = elem.offset_height() as f64;
		elem.set_width(width as u32);
		elem.set_height(height as u32);
		// get the canvas drawing context
		let context = elem.get_context("2d")?.expect(
			"context element should be supported")
		.dyn_into::<CanvasRenderingContext2d>()?;
		// get the size of each individual mino
		let size = width / 10.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#111"));
    	context.fill_rect(0.0, 0.0, width,
    		height - size * 20.0);
		context.set_fill_style(&JsValue::from_str("#222"));
    	context.fill_rect(0.0, height - size * 20.0,
    		width, size * 20.0);
    	// draw each mino
    	for y in 0..26 {
    		for x in 0..10 {
    			let pos = Position::new(x, y);
    			if let Some(mino) = self[pos] {
					context.set_fill_style(&JsValue::from_str(
						get_mino_css_color(mino)));
					context.fill_rect(
						size * x as f64,
						height - size * (y + 1) as f64,
						size + 0.5, size + 0.5);
    			}
    		}
    	}
		Ok(())
	}
}

fn draw_piece_on_board(piece: &ActivePiece, board: &Board,
		elem: &HtmlCanvasElement) -> JsResult<()> {
	// get the canvas drawing context
	let context = elem.get_context("2d")?.expect(
		"context element should be supported")
	.dyn_into::<CanvasRenderingContext2d>()?;
	// get the size of each individual mino
	let width = elem.offset_width() as f64;
	let height = elem.offset_height() as f64;
	let size = width / 10.0;
	// draw each mino
	context.set_fill_style(&JsValue::from_str(
		get_piece_css_color(piece.get_type())));
	for pos in piece.get_mino_positions() {
		context.fill_rect(
			size * pos.x as f64,
			height - size * (pos.y + 1) as f64,
			size + 0.5, size + 0.5);
	}
	context.set_global_alpha(0.5);
	for pos in piece.get_ghost(board).get_mino_positions() {
		context.fill_rect(
			size * pos.x as f64,
			height - size * (pos.y + 1) as f64,
			size + 0.5, size + 0.5);
	}
	Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	let window = web_sys::window()
		.expect("should have a window");

	let location = window.location();
	let search = location.search()?;
	let params = UrlSearchParams::new_with_str(search.as_str())?;
	let piece_type = match params.get("piece") {
		Some(string) => string.chars().next().and_then(
			|c| get_piece_from_char(c))
			.unwrap_or(PieceType::Z),
		None => PieceType::Z,
	};
	let piece = ActivePiece::spawn(piece_type,
		Rotation::Zero);

	let board = match params.get("board") {
		Some(string) => get_board_from_str(&string),
		None => Board::new(),
	};

	let document = window.document()
		.expect("window should have a document");
	let game = document.get_element_by_id("game")
		.expect("document should have a #game div");

	let elem = board.init_html(&document)?;
	game.append_child(&elem)?;
	board.update_html(&elem)?;
	draw_piece_on_board(&piece, &board, &elem)?;

	Ok(())
}