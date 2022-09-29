pub mod component;

pub mod controller;

pub mod user_prefs;

extern crate yew;
extern crate yewdux;
extern crate wasm_bindgen;
extern crate enum_map;
extern crate enumset;
extern crate web_sys;
extern crate ztrix;
extern crate serde;
extern crate instant;
extern crate gloo_timers;
extern crate rand;

use component::game_interface::GameInterface;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
	let window = web_sys::window()
		.expect("should have a window");
	let document = window.document()
		.expect("window should have a document");
	let game = document.get_element_by_id("content")
		.expect("document should have a #game div");

    yew::start_app_in_element::<GameInterface>(game);
}