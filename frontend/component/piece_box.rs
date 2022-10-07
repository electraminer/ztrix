use controller::input_handler::ButtonEvent;
use crate::component::canvas::use_canvas;
use crate::component::button::ButtonComponent;

use ztrix::game::PieceType;

use wasm_bindgen::JsValue;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
	#[prop_or_default]
	pub piece: Option<PieceType>,
	#[prop_or_default]
	pub grayed: bool,
	#[prop_or_default]
	pub speculative: bool,

	#[prop_or_default]
	pub onbutton: Callback<ButtonEvent<()>>
}

#[function_component(PieceBoxComponent)]
pub fn piece_box_component(props: &Props) -> Html {
	let piece = props.piece;
	let grayed = props.grayed;
	let canvas = use_canvas(move |canvas, context| {
		let width = canvas.offset_width() as f64;
		let height = canvas.offset_height() as f64;
		let block_size = width / 4.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#2220"));
    	context.fill_rect(0.0, 0.0, width, height);
    	// draw the mino
    	if let Some(piece) = piece {
			context.set_fill_style(
				&JsValue::from_str(match piece {
					PieceType::I => "#29D",
					PieceType::O => "#CA0",
					PieceType::J => "#03D",
					PieceType::L => "#C51",
					PieceType::S => "#1A1",
					PieceType::Z => "#C12",
					PieceType::T => "#819",
				}));
    		if grayed {
				context.set_global_alpha(0.5);
    		}
    		// position at which to put piece origin
    		let (x_offset, y_offset) = match piece {
    			PieceType::I => (-1.0, 0.5),
    			PieceType::O => (-1.0, 1.0),
    			_ => (-0.5, 0.0),
    		};
			for vec in piece.get_mino_vecs() {
				let x = vec.x as f64;
				let y = vec.y as f64;
				context.fill_rect(
					width / 2.0 + block_size * (x + x_offset),
					height / 2.0 - block_size * (y + y_offset),
					block_size + 0.5, block_size + 0.5);
			}
    	}
	});

	html! {
		<ButtonComponent onbutton={props.onbutton.clone()}>
			<canvas class="piece-box" width=0 height=0
				ref={canvas.clone()}>
			</canvas>
			{if props.speculative {
				html! {
					<img class="speculative"
						src="/assets/speculation.png"
						alt="🔁"/>
				}
			} else {
				html! { }
			}}
		</ButtonComponent>
	}
}