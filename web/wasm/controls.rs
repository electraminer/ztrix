
use std::collections::HashMap;

use enumset::EnumSetType;
use enumset::EnumSet;

use yew::prelude::*;

use Model;
use Msg;

use ztrix::position::Rotation;
use ztrix::game::Game;
use ztrix::game::Action;

#[derive(EnumSetType)]
pub enum Controls {
	Left,
	Right,
	DownSlow,
	DownFast,
	Clockwise,
	Anticlockwise,
	Flip,
	Place,
	Hold,
	Zone,
	Undo,
	Redo,
	Restart,
}

impl std::fmt::Display for Controls {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Controls::Left => write!(f, "Left"),
            Controls::Right => write!(f, "Right"),
            Controls::DownSlow => write!(f, "Down"),
            Controls::DownFast => write!(f, "Instant"),
            Controls::Clockwise => write!(f, "CW"),
            Controls::Anticlockwise => write!(f, "AntiCW"),
            Controls::Flip => write!(f, "180"),
            Controls::Place => write!(f, "Place"),
            Controls::Hold => write!(f, "Hold"),
            Controls::Zone => write!(f, "Zone"),
            Controls::Undo => write!(f, "Undo"),
            Controls::Redo => write!(f, "Redo"),
            Controls::Restart => write!(f, "Restart"),
        }
    }
}

impl Controls {
	fn get_name(&self) -> String {
        format!("{}", self)
    }

    fn get_icon_url(&self) -> String {
        format!("assets/{}.png", self).to_lowercase()
    }
}

pub struct ControlsBinding {
	keybinds: HashMap<String, Controls>,
	left_buttons: Vec<Controls>,
	right_buttons: Vec<Controls>,
	bottom_buttons: Vec<Vec<Controls>>,
}

impl ControlsBinding {
	pub fn new() -> ControlsBinding {
		ControlsBinding{
			keybinds: HashMap::from([
				("KeyK", Controls::Left),
		    	("Semicolon", Controls::Right),
		        ("KeyL", Controls::DownSlow),
		        ("ShiftLeft", Controls::DownFast),
		        ("KeyO", Controls::Clockwise),
		        ("KeyA", Controls::Anticlockwise),
		        ("KeyW", Controls::Flip),
		        ("Space", Controls::Place),
		        ("KeyD", Controls::Hold),
		        ("KeyF", Controls::Zone),
		        ("KeyZ", Controls::Undo),
		        ("KeyR", Controls::Redo),
		        ("KeyS", Controls::Restart)]
		        	.map(|(s, c)| (s.to_string(), c))),
			left_buttons: Vec::from([Controls::Restart]),
			right_buttons: Vec::from([]),
			bottom_buttons: Vec::from([
				Vec::from([
					Controls::Place,
					Controls::Place,
					Controls::Place,
					Controls::Flip]),
				Vec::from([
					Controls::Left,
					Controls::Right,
					Controls::Anticlockwise,
					Controls::Clockwise]),
				Vec::from([
					Controls::DownFast,
					Controls::DownSlow,
					Controls::Hold,
					Controls::Hold]),]),
		}
	}

	fn get_button_html(&self, ctx: &Context<Model>,
			control: Controls) -> Html {
	    html! {
	        <button
	        	onclick={ctx.link().callback(move |_|
	        		Msg::BtnClick(control))}
	        	ontouchstart={ctx.link().callback(move |_|
	        		Msg::BtnDown(control))}
	        	ontouchend={ctx.link().callback(move |_|
	        		Msg::BtnUp(control))}>
	        	<img src={control.get_icon_url()}
	        		alt={control.get_name()}/>
	        </button>
	    }
	}

	pub fn get_left_html(&self, ctx: &Context<Model>) -> Html {
		html! {
			<div class="side-buttons">{
				self.left_buttons.iter()
					.map(|&c| self.get_button_html(ctx, c))
					.collect::<Html>()
			}</div>
		}
	}

	pub fn get_right_html(&self, ctx: &Context<Model>) -> Html {
		html! {
			<div class="side-buttons">{
				self.right_buttons.iter()
					.map(|&c| self.get_button_html(ctx, c))
					.collect::<Html>()
			}</div>
		}
	}

	fn get_row_html(&self, ctx: &Context<Model>,
			buttons: &Vec<Controls>) -> Html {
	    html! {
	        <div class="button-row">{
				buttons.iter()
					.map(|&c| self.get_button_html(ctx, c))
		      		.collect::<Html>()
	    	}</div>
	    }
	}

	pub fn get_bottom_html(&self, ctx: &Context<Model>) -> Html {
		html! {
			<div class="bottom_buttons">{
				self.bottom_buttons.iter()
					.map(|c| self.get_row_html(ctx, c))
					.collect::<Html>()
			}</div>
		}
	}

	pub fn map_key(&self, key: String) -> Option<Controls> {
		self.keybinds.get(&key).map(|&c| c)
	}
}

pub struct ControlsHandler {
	pressed: EnumSet<Controls>,
}

impl ControlsHandler {

	pub fn new() -> ControlsHandler {
		ControlsHandler{pressed: EnumSet::empty()}
	}

	pub fn irs(&self) -> Rotation {
		let mut irs = Rotation::Zero;
		if self.pressed.contains(Controls::Clockwise) {
			irs = irs + Rotation::Clockwise;
		}
		if self.pressed.contains(Controls::Anticlockwise) {
			irs = irs + Rotation::Anticlockwise;
		}
		return irs;
	}

	pub fn press(&mut self, game: &mut Game, control: Controls) -> bool {
		self.pressed.insert(control) && match control {
    		Controls::Left => game.execute(
	    		Action::MoveLeft),
		    Controls::Right => game.execute(
	    		Action::MoveRight),
		   	Controls::DownSlow => game.execute(
	    		Action::MoveDown),
		   	Controls::DownFast => {
		   		while game.execute(Action::MoveDown) {};
		   		true},
		    Controls::Clockwise => game.execute(
	    		Action::Rotate(Rotation::Clockwise)),
		    Controls::Anticlockwise => game.execute(
	    		Action::Rotate(Rotation::Anticlockwise)),
		    Controls::Flip => game.execute(
	    		Action::Rotate(Rotation::Flip)),
		    Controls::Place => {
	        	game.execute(Action::PlacePiece(
	        		self.irs(), self.pressed.contains(Controls::Hold)));
        		game.execute(Action::SpawnPiece(
        			self.irs(), self.pressed.contains(Controls::Hold)));
        		true},
	    	Controls::Hold => game.execute(
	    		Action::HoldPiece(self.irs())),
	    	Controls::Restart => {
				let mut g = Game::new();
				g.execute(Action::SpawnPiece(
					self.irs(), self.pressed.contains(Controls::Hold)));
	    		*game = g;
	    		true},
    		_ => false,
		}
	}

	pub fn release(&mut self, _game: &Game, control: Controls) -> bool {
		self.pressed.remove(control)
	}
}