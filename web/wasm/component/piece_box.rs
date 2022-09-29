

use ztrix::game::PieceType;
use wasm_bindgen::JsCast;
use component::subcomponent::Subcomponent;
use component::game_interface::GameInterface;

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use yew::prelude::*;
use wasm_bindgen::JsValue;

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

pub struct PieceBoxSubcomponent {
	canvas: NodeRef,
}

pub struct Props {
	pub piece: Option<PieceType>,
	pub grayed: bool,
}

impl Subcomponent for PieceBoxSubcomponent {
	type Component = GameInterface;
	type Properties<'a> = Props;

	fn new() -> PieceBoxSubcomponent {
		PieceBoxSubcomponent{
			canvas: NodeRef::default(),
		}
	}

	fn view(&self, _ctx: &Context<Self::Component>,
			_props: Props) -> Html {
		html! {
			<canvas class="piece-box" width=0 height=0
				ref={self.canvas.clone()}>
			</canvas>
		}
	}

	fn rendered(&self, _ctx: &Context<Self::Component>,
			props: Props, _first: bool) {
		// get hold canvas
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
		let size = width / 4.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#222"));
    	context.fill_rect(0.0, 0.0, width, height);
    	// draw the hold mino
    	if let Some(piece) = props.piece {
    		if props.grayed {
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
    }
}