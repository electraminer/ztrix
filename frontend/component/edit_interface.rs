
use controller::input_handler::ButtonHandler;
use user_prefs::UserPrefs;
use std::str::FromStr;
use web_sys::HtmlInputElement;
use component::keyboard_interface::KeyboardInterface;
use controller::input_handler::ButtonEvent;
use yew_router::prelude::*;
use component::button::ButtonComponent;
use enumset::EnumSet;
use ztrix::game::MaybeActive;
use component::piece_box::PieceBoxComponent;
use component::game::GameButton;
use component::queue::QueueButton;
use component::board::BoardMouseEvent;
use crate::component::game::GameComponent;
use crate::component::router::Route;

use yew::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use ztrix::game::Game;
use ztrix::game::Mino;
use ztrix::game::PieceType;

fn cycle_piece(piece: PieceType) -> PieceType {
	match piece {
		PieceType::I => PieceType::O,
		PieceType::O => PieceType::J,
		PieceType::J => PieceType::L,
		PieceType::L => PieceType::S,
		PieceType::S => PieceType::Z,
		PieceType::Z => PieceType::T,
		PieceType::T => PieceType::I,
	}
}

fn update_bag(game: &mut Game, advance: usize) {
	let mut bag_pos = game.queue.rando.set.len() + advance;
	if bag_pos > 7 {
		bag_pos -= 7;
	}
	let c = 7 - 4 - 1;
	let used: EnumSet<PieceType> =
		(bag_pos..7)
		.filter_map(|n| if n == c {
			Some(game.get_current())
		} else if n > c {
			Some(game.queue[n - c - 1])
		} else {
			None
		}).collect();
	game.queue.rando.set =
		EnumSet::all().iter()
		.filter(|p| !used.contains(*p))
		.take(bag_pos).collect();
}


#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum EditButton {
	SetHold,
	SetCurrent,
	SetNext(usize),
	SetBagPos,
	ToggleZone,
	ToggleHoldUsed,
	Play,
	Import,
	Export,
	Revert,
	EraseAll,
}

impl EditButton {
	pub fn get_name(&self) -> String {
        match self {
            EditButton::SetHold => "Set Hold",
            EditButton::SetCurrent => "Set Current",
            EditButton::SetNext(n) =>
            	return format!{"Set Next #{}", n},
            EditButton::SetBagPos => "Set Bag Position",
            EditButton::ToggleZone => "Toggle Zone",
            EditButton::ToggleHoldUsed => "Toggle Hold Used",
            EditButton::Play => "Enter Play Mode",
            EditButton::Import => "Import Link",
            EditButton::Export => "Export Link",
            EditButton::Revert => "Revert Changes",
            EditButton::EraseAll => "Erase All",
        }.to_string()
    }
}

pub enum Msg {
	KeyButton(ButtonEvent<String>),
	GameButton(ButtonEvent<GameButton>),
	Button(ButtonEvent<EditButton>),
	Draw(BoardMouseEvent),
}

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
}

pub struct EditInterface {
	initial: Game,
	game: Game,
	brush: Option<Mino>,
	input: NodeRef,
	button_handler: ButtonHandler<EditButton>,
}

impl Component for EditInterface {
	type Message = Msg;
	type Properties = Props;

	fn create(ctx: &Context<Self>) -> Self {
		let props = ctx.props();
		Self {
			initial: props.game.clone(),
			game: props.game.clone(),
			brush: None,
			input: NodeRef::default(),
			button_handler: ButtonHandler::new(),
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
        	<KeyboardInterface
        		onkey={ctx.link().callback(
        			|e: ButtonEvent<String>|
        				Msg::KeyButton(e))}>
	      		<GameComponent game={self.game.clone()}
	      			onboardmouse={ctx.link().callback(
						move |e: BoardMouseEvent|
							Msg::Draw(e))}
	      			onbutton={ctx.link().callback(
						move |e: ButtonEvent<GameButton>|
							Msg::GameButton(e))}
	      			top_left={{ html!{ <>
		        		<p><strong>{"CURRENT"}</strong></p>
		        		<hr class="spacer"/>
		        		<PieceBoxComponent
							piece={self.game.get_current()}
							onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| EditButton::SetCurrent)))}/>
	      			</> }}}
	      			bottom_left={{ html! { <>
		        		<ButtonComponent
		        			onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| EditButton::ToggleHoldUsed)))}>
		        			<img src="/assets/hold.png"
		        				alt="Toggle Hold Used"/>
		        		</ButtonComponent>
		        		<ButtonComponent
		        			onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| EditButton::ToggleZone)))}>
		        			<img src="/assets/zone.png"
		        				alt="Toggle Zone"/>
		        		</ButtonComponent>
	      			</> }}}
	      			top_right={{ html! { <>
		        		<ButtonComponent
		        			onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| EditButton::Play)))}>
		        			<img src="/assets/play.png"
		        				alt="Enter Play Mode"/>
		        		</ButtonComponent>
	      			</> }}}/>
	      		<div class="row">
	      			<input type="text"
	      				ref={self.input.clone()}
	      				placeholder="https://ztrix-game.web.app/game/..."/>
	      		</div>
	      		<div class="row">
	      			<ButtonComponent
	      				onbutton={ctx.link().callback(
							move |e: ButtonEvent<()>|
								Msg::Button(e.map(|_|
									EditButton::Revert)))}>
	      				<img src="/assets/revert.png"
		        				alt="Revert"/>
	      			</ButtonComponent>
	      			<ButtonComponent
	      				onbutton={ctx.link().callback(
							move |e: ButtonEvent<()>|
								Msg::Button(e.map(|_|
									EditButton::EraseAll)))}>
	      				<img src="/assets/eraseall.png"
		        				alt="Erase All"/>
	      			</ButtonComponent>
	      			<ButtonComponent
	      				onbutton={ctx.link().callback(
							move |e: ButtonEvent<()>|
								Msg::Button(e.map(|_|
									EditButton::Import)))}>
	      				<img src="/assets/import.png"
		        				alt="Import"/>
	      			</ButtonComponent>
	      			<ButtonComponent
	      				onbutton={ctx.link().callback(
							move |e: ButtonEvent<()>|
								Msg::Button(e.map(|_|
									EditButton::Export)))}>
	      				<img src="/assets/export.png"
		        				alt="Export"/>
						<div class="copied">
							<p>{"Copied link!"}</p>
						</div>
	      			</ButtonComponent>
	      		</div>
	        </KeyboardInterface>
	    }
	}

	fn update(&mut self, ctx: &Context<Self>,
			msg: Self::Message) -> bool {
		let user_prefs = UserPrefs::get();
    	let key_bindings = &user_prefs.key_bindings;
    	let event = match msg {
    		Msg::KeyButton(event) =>
    			match event.maybe_map(|b|
    				key_bindings.edit_bindings.get(&b)
    				.copied())
    				.and_then(|e| self.button_handler
    					.update(e)) {
					Some(event) => event,
					None => return false,
				}
    		Msg::GameButton(event) =>
    			match self.button_handler.update(
    				event.map(|b| match b {
						GameButton::Hold =>
							EditButton::SetHold,
						GameButton::Queue(
							QueueButton::NextBox(n)) =>
								EditButton::SetNext(n+1),
						GameButton::Queue(
							QueueButton::BagInfo) =>
								EditButton::SetBagPos,
					})) {
					Some(event) => event,
					None => return false,
				}
    		Msg::Button(event) => match self.button_handler
    				.update(event) {
					Some(event) => event,
					None => return false,
    			}
			Msg::Draw(e) => {
				match e {
					BoardMouseEvent::Press(pos) => {
						let board = &mut self.game.board;
						self.brush = match board[pos] {
							Some(_) => None,
							None => Some(Mino::Gray),
						};
						board[pos] = self.brush;
					},
					BoardMouseEvent::Move(pos) => {
						let board = &mut self.game.board;
						board[pos] = self.brush;
					},
					BoardMouseEvent::Release => (),
				}
				return true;
			}
		};
		match event {
			ButtonEvent::Press(b) => match b {
				EditButton::SetHold => {
					let hold = &mut self.game.hold;
					*hold = match *hold {
						Some(PieceType::T) => None,
						Some(p) => Some(cycle_piece(p)),
						None => Some(PieceType::I),
					}
				},
				EditButton::SetCurrent => {
					let piece = &mut self.game.piece;
					*piece = MaybeActive::Inactive(
						cycle_piece(piece.get_type()));
					update_bag(&mut self.game, 0);
				},
				EditButton::SetNext(n) => {
					let queue = &mut self.game.queue;
					queue[n-1] = cycle_piece(queue[n-1]);
					update_bag(&mut self.game, 0);
				},
				EditButton::SetBagPos => {
					update_bag(&mut self.game, 1);
				},
				EditButton:: ToggleHoldUsed => {
					self.game.has_held = !self.game.has_held;
				},
				EditButton::ToggleZone => {
					self.game.in_zone = !self.game.in_zone;
				},
				EditButton::Import => {
					let input = self.input
						.cast::<HtmlInputElement>()
		    			.expect("element should be an input");
	    			let value = input.value();
	    			let prefix1 = vec![
	    				"https://", "http://", ""];
	    			let prefix2 = vec![
	    				"ztrix-game.web.app/",
	    				"152.7.71.114/",
	    				"localhost/", "/", ""];
	    			let prefix3 = vec![
	    				"game/", "play/", "edit/", ""];
	    			let mut code = None;
	    			for p1 in prefix1.iter() {
	    				for p2 in prefix2.iter() {
	    					for p3 in prefix3.iter() {
	    						let prefix = format!{
	    							"{}{}{}", p1, p2, p3};
	    						code = code.or_else(
	    							|| value.strip_prefix(&prefix));
	    					}
	    				}
	    			}
	    			match code.and_then(|c|
	    				Game::from_str(c).ok()) {
	    				Some(game) => {
	    					input.set_custom_validity("");
	    					self.game = game;
	    				}
	    				None => {
	    					input.set_custom_validity(
	    						"Not a valid link!");
	    				}
	    			}
				},
				EditButton::Export => {
					let input = self.input
						.cast::<HtmlInputElement>()
		    			.expect("element should be an input");
		    		let value = format!{
		    			"https://ztrix-game.web.app/game/{}", self.game};
		    		input.set_value(&value);
					let window = web_sys::window()
						.expect("should be a window");
					let navigator = window.navigator();
					if let Some(clipboard) = navigator.clipboard() {
						let _ = clipboard.write_text(&value);
					}
				},
				EditButton::Revert =>
					self.game = self.initial.clone(),
				EditButton::EraseAll =>
					self.game = Game::default(),
				_ => (),
			}
			ButtonEvent::Release(b) => match b {		
				EditButton::Play => {
					let history = ctx.link().history()
						.expect("should be a history");
					history.push(
						Route::GameGame {
							game: self.game.clone()
						}
					)
				}
				_ => (),
			}
		}
		true
	}
}