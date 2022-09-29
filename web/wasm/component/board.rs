

use ztrix::game::Mino;
use ztrix::game::Board;
use ztrix::game::ActivePiece;
use ztrix::position::Position;
use ztrix::position::Rotation;
use ztrix::game::PieceType;
use wasm_bindgen::JsCast;
use component::subcomponent::Subcomponent;
use component::game_interface::GameInterface;

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use yew::prelude::*;
use wasm_bindgen::JsValue;

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

pub struct BoardSubcomponent {
	canvas: NodeRef,
}

pub struct Props<'a> {
	pub board: &'a Board,
	pub piece: &'a Option<ActivePiece>,
	pub current: PieceType,
}

impl Subcomponent for BoardSubcomponent {
	type Component = GameInterface;
	type Properties<'a> = Props<'a>;

	fn new() -> BoardSubcomponent {
		BoardSubcomponent{
			canvas: NodeRef::default(),
		}
	}

	fn view<'a>(&self, _ctx: &Context<Self::Component>,
			_props: Props<'a>) -> Html {
		html! {
			<canvas class="board" width=0 height=0
				ref={self.canvas.clone()}>
			</canvas>
		}
	}

	fn rendered<'a>(&self, _ctx: &Context<Self::Component>,
			props: Props<'a>, _first: bool) {
		// get board canvas
        let canvas = self.canvas.cast::<HtmlCanvasElement>()
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
    		if y == 20 {
				context.set_global_alpha(0.75);
    		}
    		for x in 0..10 {
    			let pos = Position::new(x, y);
    			if let Some(mino) = props.board[pos] {
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
		if let Some(piece) = props.piece {
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
			for pos in piece.get_ghost(props.board)
				.get_mino_positions() {
				context.fill_rect(
					size * pos.x as f64,
					height - size * (pos.y + 1) as f64,
					size + 0.5, size + 0.5);
			}
		} else {
			context.set_fill_style(&JsValue::from_str(
				get_piece_css_color(props.current)));
			context.set_global_alpha(0.5);
			let piece = ActivePiece::spawn_unchecked(
				props.current, Rotation::Zero);
			for pos in piece.get_mino_positions() {
				context.fill_rect(
					size * pos.x as f64,
					height - size * (pos.y + 1) as f64,
					size + 0.5, size + 0.5);
			}
		}
    }
}