use wasm_bindgen::JsCast;

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use yew::prelude::*;

pub fn use_canvas<F>(func: F) -> NodeRef
where 	F: Fn(HtmlCanvasElement, CanvasRenderingContext2d)
			+ 'static {
	let node_ref = use_node_ref();
	{
		let node_ref = node_ref.clone();
		use_effect(move || {
		    let canvas = node_ref.cast::<HtmlCanvasElement>()
		    	.expect("element should be a canvas");
			let width = canvas.offset_width();
			let height = canvas.offset_height();
			canvas.set_width(width as u32);
			canvas.set_height(height as u32);
			// get rendering context
			let context = canvas.get_context("2d")
				.expect("canvas should have context")
				.expect("context element should be supported")
			.dyn_into::<CanvasRenderingContext2d>()
				.expect("element should be a context");
			func(canvas, context);
			|| ()
		});
	}
	node_ref
}