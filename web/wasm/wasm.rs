

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate yew;
use yew::prelude::*;

extern crate ztrix;
use ztrix::board::Board;
use ztrix::position::position::Position;
use ztrix::position::vector::Vector;
use ztrix::position::rotation::Rotation;
use ztrix::piece::piece_type::PieceType;
use ztrix::piece::active_piece::ActivePiece;
use ztrix::mino::Mino;

extern crate web_sys;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::UrlSearchParams;

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

#[derive(Copy, Clone)]
enum Msg {
	MoveLeft,
	MoveRight,
	MoveDown,
}

struct Model {
	board_canvas: NodeRef,
    board: Board,
    piece: ActivePiece,
}

fn controls(string: &str) -> Option<Msg> {
    match string {
    	"KeyK" => Some(Msg::MoveLeft),
    	"Semicolon" => Some(Msg::MoveRight),
        "ShiftLeft" => Some(Msg::MoveDown),
        _ => None
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
    	// get the url parameters
    	let window = web_sys::window()
			.expect("should have a window");
		let location = window.location();
		let search = location.search()
			.expect("location should have a search");
		let params = UrlSearchParams::new_with_str(
				search.as_str())
			.expect("search should be valid parameters");
		// generate board from parameters
		let board = match params.get("board") {
			Some(string) => get_board_from_str(&string),
			None => Board::new(),
		};
		// generate active piece from parameter
		let piece_type = match params.get("piece") {
			Some(string) => string.chars().next().and_then(
				|c| get_piece_from_char(c))
				.unwrap_or(PieceType::Z),
			None => PieceType::Z,
		};
		let piece = ActivePiece::spawn(piece_type,
			Rotation::Zero);
		// create model
        Self {
    		board_canvas: NodeRef::default(),
            board: board,
            piece: piece,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,
    		msg: Msg) -> bool {
    	match msg {
    		Msg::MoveLeft => self.piece.try_move(
    				&self.board, Vector::ONE_LEFT),
    		Msg::MoveRight => self.piece.try_move(
    				&self.board, Vector::ONE_RIGHT),
    		Msg::MoveDown => self.piece.try_move(
    				&self.board, Vector::ONE_DOWN),
    	}
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
    	let callback = ctx.link()
    		.batch_callback(move |e: KeyboardEvent| {
    			controls(e.code().as_str())
    		});
        html! {
            <canvas class="board"
            	ref={self.board_canvas.clone()}
            	tabindex=1
            	onkeydown={callback}>
            </canvas>
        }
    }

     fn rendered(&mut self, _ctx: &Context<Self>,
     		_first: bool) {
        // update canvas size
        let canvas = self.board_canvas.cast::<HtmlCanvasElement>()
        	.expect("element should be a canvas");
		let width = canvas.offset_width() as f64;
		let height = canvas.offset_height() as f64;
		canvas.set_width(width as u32);
		canvas.set_height(height as u32);
		// get rendering context
		let context = canvas.get_context("2d")
			.expect("canvas should have context")
			.expect("context element should be supported")
		.dyn_into::<CanvasRenderingContext2d>()
			.expect("element should be a context");
		// get the size of each individual mino
		let size = width / 10.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#111"));
    	context.fill_rect(0.0, 0.0, width,
    		height - size * 20.0);
		context.set_fill_style(&JsValue::from_str("#222"));
    	context.fill_rect(0.0, height - size * 20.0,
    		width, size * 20.0);
    	// draw each board mino
    	for y in 0..26 {
    		for x in 0..10 {
    			let pos = Position::new(x, y);
    			if let Some(mino) = self.board[pos] {
					context.set_fill_style(&JsValue::from_str(
						get_mino_css_color(mino)));
					context.fill_rect(
						size * x as f64,
						height - size * (y + 1) as f64,
						size + 0.5, size + 0.5);
    			}
    		}
    	}
		// draw the active piece
		context.set_fill_style(&JsValue::from_str(
			get_piece_css_color(self.piece.get_type())));
		for pos in self.piece.get_mino_positions() {
			context.fill_rect(
				size * pos.x as f64,
				height - size * (pos.y + 1) as f64,
				size + 0.5, size + 0.5);
		}
		// draw the ghost piece
		context.set_global_alpha(0.5);
		for pos in self.piece.get_ghost(&self.board)
			.get_mino_positions() {
			context.fill_rect(
				size * pos.x as f64,
				height - size * (pos.y + 1) as f64,
				size + 0.5, size + 0.5);
		}
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
	let window = web_sys::window()
		.expect("should have a window");
	let document = window.document()
		.expect("window should have a document");
	let game = document.get_element_by_id("game")
		.expect("document should have a #game div");
    yew::start_app_in_element::<Model>(game);
}