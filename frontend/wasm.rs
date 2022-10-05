pub mod component;

pub mod controller;

pub mod user_prefs;

extern crate yew;
extern crate yewdux;
extern crate yew_router;
extern crate wasm_bindgen;
extern crate enum_map;
extern crate enumset;
extern crate web_sys;
extern crate ztrix;
extern crate serde;
extern crate instant;
extern crate gloo_timers;
extern crate rand;
use component::router::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}