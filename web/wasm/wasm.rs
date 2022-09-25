extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate yew;
use yew::prelude::*;

extern crate ztrix;
use ztrix::game::Action;

use ztrix::position::Position;

use ztrix::position::Rotation;
use ztrix::piece::PieceType;

use ztrix::mino::Mino;

use ztrix::game::Game;

extern crate web_sys;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::UrlSearchParams;

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

enum Msg {
	KeyUpdate(String),
}

struct Model {
	game: Game,
	board_canvas: NodeRef,
	queue_canvas: NodeRef,
	hold_canvas: NodeRef,
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
		let _params = UrlSearchParams::new_with_str(
				search.as_str())
			.expect("search should be valid parameters");
		let mut game = Game::new();
		game.execute(Action::SpawnPiece(
			Rotation::Zero, false));
		// create model
        Self {
        	game: game,
    		board_canvas: NodeRef::default(),
    		hold_canvas: NodeRef::default(),
    		queue_canvas: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,
    		msg: Msg) -> bool {
    	match msg {
    		Msg::KeyUpdate(key) => match key.as_str() {
		    	"KeyK" => self.game.execute(
		    		Action::MoveLeft),
		    	"Semicolon" => self.game.execute(
		    		Action::MoveRight),
		        "ShiftLeft" => self.game.execute(
		    		Action::MoveDown),
		    	"KeyO" => self.game.execute(
		    		Action::Rotate(Rotation::Clockwise)),
		    	"KeyA" => self.game.execute(
		    		Action::Rotate(Rotation::Anticlockwise)),
		        "KeyD" =>  self.game.execute(
		    		Action::HoldPiece(Rotation::Zero)),
		        "Space" => {
		        	self.game.execute(Action::PlacePiece(
		        		Rotation::Zero, false));
	        		self.game.execute(Action::SpawnPiece(
	        			Rotation::Zero, false));
	        		true},
		        _ => false,
		    }
    	}
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
    	let callback = ctx.link().callback(
    			move |e: KeyboardEvent|
    				Msg::KeyUpdate(e.code()));
        html! {
        	<div class="game"
            	tabindex=1
            	onkeydown={callback}>
            	<div class="sidebar">
	        		<canvas class="hold"
		            	ref={self.hold_canvas.clone()}>
		            </canvas>
	            </div>
	            <canvas class="board"
	            	ref={self.board_canvas.clone()}>
	            </canvas>
            	<div class="sidebar">
		            <canvas class="queue"
		            	ref={self.queue_canvas.clone()}>
		            </canvas>
	            </div>
            </div>
        }
    }

     fn rendered(&mut self, _ctx: &Context<Self>,
     		_first: bool) {
        // get board canvas
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
    	let board = self.game.get_board();
    	for y in 0..26 {
    		if y == 20 {
				context.set_global_alpha(0.75);
    		}
    		for x in 0..10 {
    			let pos = Position::new(x, y);
    			if let Some(mino) = board[pos] {
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
		if let Some(piece) = self.game.get_piece() {
			context.set_global_alpha(1.0);
			context.set_fill_style(&JsValue::from_str(
				get_piece_css_color(piece.get_type())));
			for pos in piece.get_mino_positions() {
				context.fill_rect(
					size * pos.x as f64,
					height - size * (pos.y + 1) as f64,
					size + 0.5, size + 0.5);
			}
			// draw the ghost piece
			context.set_global_alpha(0.5);
			for pos in piece.get_ghost(board)
				.get_mino_positions() {
				context.fill_rect(
					size * pos.x as f64,
					height - size * (pos.y + 1) as f64,
					size + 0.5, size + 0.5);
			}
		}
		// get hold canvas
        let canvas = self.hold_canvas.cast::<HtmlCanvasElement>()
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
		let size = width / 4.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#222"));
    	context.fill_rect(0.0, 0.0, width, height);
    	// draw the hold mino
    	if let Some(piece) = self.game.get_hold() {
    		if self.game.get_held() {
				context.set_global_alpha(0.5);
    		} else {
    			context.set_global_alpha(1.0);
    		}
			context.set_fill_style(&JsValue::from_str(
				get_piece_css_color(piece)));
			let mut x_offset = -0.5;
			let mut y_offset = 0.0;
			if piece == PieceType::I {
				x_offset = -1.0;
				y_offset = 0.5;
			}
			if piece == PieceType::O {
				x_offset = -1.0;
				y_offset = 1.0;
			}
			for vec in piece.get_mino_vecs() {
				context.fill_rect(
					width / 2.0 + size * (vec.x as f64 + x_offset),
					height / 2.0 - size * (vec.y as f64 + y_offset),
					size + 0.5, size + 0.5);
			}
    	}
    	// get queue canvas
        let canvas = self.queue_canvas.cast::<HtmlCanvasElement>()
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
		let size = width / 4.0;
    	// draw the hold mino
    	context.set_global_alpha(1.0);
    	for i in 0..4 {
			// draw the background
			context.set_fill_style(&JsValue::from_str("#222"));
	    	context.fill_rect(0.0, i as f64 * height / 4.0, width, size * 3.0);
    		let piece = self.game.get_queue().get(i);
			context.set_fill_style(&JsValue::from_str(
				get_piece_css_color(piece)));
			let mut x_offset = -0.5;
			let mut y_offset = 0.0;
			if piece == PieceType::I {
				x_offset = -1.0;
				y_offset = 0.5;
			}
			if piece == PieceType::O {
				x_offset = -1.0;
				y_offset = 1.0;
			}
			for vec in piece.get_mino_vecs() {
				context.fill_rect(
					width / 2.0 + size * (vec.x as f64 + x_offset),
					size * 1.5 + height / 4.0 * i as f64 - size * (vec.y as f64 + y_offset),
					size + 0.5, size + 0.5);
			}
    	}
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
	let window = web_sys::window()
		.expect("should have a window");
	let document = window.document()
		.expect("window should have a document");
	let game = document.get_element_by_id("content")
		.expect("document should have a #game div");
    yew::start_app_in_element::<Model>(game);
}