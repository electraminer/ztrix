
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

pub enum EditButton {
	SetHold,
	SetCurrent,
	SetNext(usize),
	SetBagPos,
	ToggleZone,
	Play,
	Import,
	Export,
	Revert,
	EraseAll,
}

impl From<GameButton> for EditButton {
	fn from(button: GameButton) -> Self {
		match button {
			GameButton::Hold => EditButton::SetHold,
			GameButton::Queue(QueueButton::NextBox(n)) =>
				EditButton::SetNext(n),
			GameButton::Queue(QueueButton::BagInfo) =>
				EditButton::SetBagPos,
			
		}
	}
}

pub enum Msg {
	Draw(BoardMouseEvent),
	Button(ButtonEvent<EditButton>),
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
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
        	<KeyboardInterface>
	      		<GameComponent game={self.game.clone()}
	      			onboardmouse={ctx.link().callback(
						move |e: BoardMouseEvent|
							Msg::Draw(e))}
	      			onbutton={ctx.link().callback(
						move |e: ButtonEvent<GameButton>|
							Msg::Button(e.map(|b|
								EditButton::from(b))))}
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
		        			<p>{"Play"}</p>
		        		</ButtonComponent>
	      			</> }}}/>
	      		<div class="row">
	      			<input type="text"
	      				ref={self.input.clone()}
	      				placeholder="http://152.7.71.114/game/..."/>
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
	      			</ButtonComponent>
	      		</div>
	        </KeyboardInterface>
	    }
	}

	fn update(&mut self, ctx: &Context<Self>,
			msg: Self::Message) -> bool {
		match msg {
			Msg::Draw(e) => match e {
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
			Msg::Button(e) => match e {
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
						queue[n] = cycle_piece(queue[n]);
						update_bag(&mut self.game, 0);
					},
					EditButton::SetBagPos => {
						update_bag(&mut self.game, 1);
					},
					EditButton::ToggleZone => {
						self.game.in_zone = !self.game.in_zone;
					},
					EditButton::Play => {
						let history = ctx.link().history()
							.expect("should be a history");
						history.push(
							Route::GameGame {
								game: self.game.clone()
							}
						)
					}
					EditButton::Import => {
						let input = self.input
							.cast::<HtmlInputElement>()
			    			.expect("element should be an input");
		    			let value = input.value();
		    			let prefix1 = vec![
		    				"https://", "http://", ""];
		    			let prefix2 = vec![
		    				"152.7.71.114/",
		    				"localhost/",
		    				"ztrix/", "/", ""];
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
			    			"http://152.7.71.114/game/{}", self.game};
			    		input.set_value(&value);
						input.select();
						input.set_selection_range(0, 99999)
							.expect("should be able to select");
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
				}
				ButtonEvent::Release(_) => (),
			}
		}
		true
	}
}