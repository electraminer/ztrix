use std::rc::Rc;
use web_sys::HtmlElement;
use crate::component::canvas::use_canvas;

use ztrix::position::Rotation;
use ztrix::position::Position;
use ztrix::game::PieceType;
use ztrix::game::Mino;
use ztrix::game::Board;
use ztrix::game::ActivePiece;
use ztrix::game::MaybeActive;

use wasm_bindgen::JsValue;

use yew::prelude::*;

#[derive(Debug)]
pub enum BoardMouseEvent {
	Press(Position),
	Move(Position),
	Release,
}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub board: Board,
	#[prop_or_default]
	pub piece: Option<MaybeActive>,

	#[prop_or_default]
	pub onmouse: Callback<BoardMouseEvent>
}

#[function_component(BoardComponent)]
pub fn board(props: &Props) -> Html {
	let state_position = use_state(|| None);

	let board = props.board.clone();
	let piece = props.piece.clone();
	let canvas = use_canvas(move |canvas, context| {
		let width = canvas.offset_width() as f64;
		let height = canvas.offset_height() as f64;
		let block_size = width / 10.0;
		// draw the background
		context.set_fill_style(&JsValue::from_str("#111"));
    	context.fill_rect(0.0, 0.0,
    		width, height - block_size * 20.0);
		context.set_fill_style(&JsValue::from_str("#222"));
    	context.fill_rect(0.0, height - block_size * 20.0,
    		width, block_size * 20.0);
    	// draw each board mino
    	for y in 0..26 {
    		if y == 20 {
				context.set_global_alpha(0.75);
    		}
    		for x in 0..10 {
    			let pos = Position::new(x, y);
    			if let Some(mino) = board[pos] {
					context.set_fill_style(&JsValue::from_str(
						match mino {
							Mino::Piece(PieceType::I) => "#06A",
							Mino::Piece(PieceType::O) => "#870",
							Mino::Piece(PieceType::J) => "#01B",
							Mino::Piece(PieceType::L) => "#730",
							Mino::Piece(PieceType::S) => "#070",
							Mino::Piece(PieceType::Z) => "#700",
							Mino::Piece(PieceType::T) => "#607",
							Mino::Gray => "#666",
						}));
					context.fill_rect(
						block_size * x as f64,
						height - block_size * (y + 1) as f64,
						block_size + 0.5, block_size + 0.5);
    			}
    		}
    	}
		// draw the active piece
		if let Some(piece) = &piece {
			context.set_fill_style(&JsValue::from_str(
				match piece.get_type() {
					PieceType::I => "#29D",
					PieceType::O => "#CA0",
					PieceType::J => "#03D",
					PieceType::L => "#C51",
					PieceType::S => "#1A1",
					PieceType::Z => "#C12",
					PieceType::T => "#819",
				}));
			match piece {
				MaybeActive::Active(active) => {
					context.set_global_alpha(1.0);
					for pos in active.get_mino_positions() {
						let x = pos.x as f64;
						let y = pos.y as f64;
						context.fill_rect(
							block_size * x as f64,
							height - block_size * (y + 1.0),
							block_size + 0.5, block_size + 0.5);
					}
					context.set_global_alpha(0.3);
					for pos in active.get_ghost(&board)
						.get_mino_positions() {
						let x = pos.x as f64;
						let y = pos.y as f64;
						context.fill_rect(
							block_size * x,
							height - block_size * (y + 1.0),
							block_size + 0.5, block_size + 0.5);
					}
				}
				MaybeActive::Inactive(current) => {
					context.set_global_alpha(0.3);
					let active = ActivePiece::spawn_unchecked(
						*current, Rotation::Zero);
					for pos in active.get_mino_positions() {
						let x = pos.x as f64;
						let y = pos.y as f64;
						context.fill_rect(
							block_size * x,
							height - block_size * (y + 1.0),
							block_size + 0.5, block_size + 0.5);
					}
				}
			}	
		}
	});

	let node_ref = canvas.clone();
	let get_position = Rc::new(move |x, y| {
		let rect = node_ref.cast::<HtmlElement>()
	    	.expect("node ref should be an html element")
	    	.get_bounding_client_rect();
	    let cx = rect.x() as f64;
	    let cy = (rect.y() + rect.height()) as f64;
	    let cs = rect.width() as f64 / 10.0;
		Position::new(
			((x - cx) / cs) as i32,
			((cy - y) / cs) as i32)
	});

	let onmouse = props.onmouse.clone();
	let position = state_position.clone();
	let get_pos = get_position.clone();
	let onmousedown = Callback::from(move |e: MouseEvent|
		if e.button() == 0 {
			let pos = get_pos(
				e.client_x() as f64,
				e.client_y() as f64);
			if Board::in_bounds(pos) {
				position.set(Some(pos));
				onmouse.emit(BoardMouseEvent::Press(pos));
			}
		});
	let onmouse = props.onmouse.clone();
	let position = state_position.clone();
	let get_pos = get_position.clone();
	let onmousemove = Callback::from(move |e: MouseEvent|
		if e.buttons() % 2 == 1 {
			if let Some(old_pos) = *position {
				let pos = get_pos(
					e.client_x() as f64,
					e.client_y() as f64);
				if pos != old_pos &&
					Board::in_bounds(pos) {
					position.set(Some(pos));
					onmouse.emit(BoardMouseEvent::Move(pos));
				}
			}
		} else {
			position.set(None);
		});
	let position = state_position.clone();
	let onmouseup = Callback::from(move |e: MouseEvent|
		if e.button() == 0 {
			position.set(None);
		});
	let onmouse = props.onmouse.clone();
	let position = state_position.clone();
	let get_pos = get_position.clone();
	let ontouchstart = Callback::from(move |e: TouchEvent| {
		e.prevent_default();
		let touch = e.target_touches().get(0)
			.expect("should be at least one touch");
		let pos = get_pos(
			touch.client_x() as f64,
			touch.client_y() as f64);
		if Board::in_bounds(pos) {
			position.set(Some(pos));
			onmouse.emit(BoardMouseEvent::Press(pos));
		}
	});
	let onmouse = props.onmouse.clone();
	let position = state_position.clone();
	let get_pos = get_position.clone();
	let ontouchmove = Callback::from(move |e: TouchEvent| {
		e.prevent_default();
		let touch = e.target_touches().get(0)
			.expect("should be at least one touch");
		if let Some(old_pos) = *position {
			let pos = get_pos(
				touch.client_x() as f64,
				touch.client_y() as f64);
			if pos != old_pos &&
				Board::in_bounds(pos) {
				position.set(Some(pos));
				onmouse.emit(BoardMouseEvent::Move(pos));
			}
		}
	});
	let position = state_position.clone();
	let ontouchend = Callback::from(move |e: TouchEvent| {
		e.prevent_default();
		position.set(None);
	});
	html! {
		<canvas class="board" width=0 height=0
			ref={canvas.clone()}
        	{onmousedown} {onmousemove} {onmouseup}
        	{ontouchstart} {ontouchmove} {ontouchend}>
		</canvas>
	}
}