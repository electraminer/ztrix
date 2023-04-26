
use std::collections::VecDeque;
use controller::input_handler::ButtonHandler;
use user_prefs::UserPrefs;
use ztrix::replay::Replay;
use ztrix::replay::Solution;
use ztrix::serialize::DeserializeInput;
use ztrix::serialize::SerializeUrlSafe;
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
use ztrix::puzzle::Puzzle;

use yew::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use ztrix::game::Game;
use ztrix::game::Mino;
use ztrix::game::PieceType;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
#[derive(Debug)]
pub enum ReplayButton {
	GoToStart,
	GoToEnd,
	Forward,
	Back,
	Import,
	Export,
	Play,
	Edit,
}

impl ReplayButton {
	pub fn get_name(&self) -> String {
        match self {
            ReplayButton::GoToStart => "Go To Start",
            ReplayButton::GoToEnd => "Go To End",
            ReplayButton::Forward => "Forward",
            ReplayButton::Back => "Back",
            ReplayButton::Play => "Enter Play Mode",
            ReplayButton::Edit => "Enter Edit Mode",
            ReplayButton::Import => "Import Link",
            ReplayButton::Export => "Export Link",
        }.to_string()
    }
}

pub enum Msg {
	KeyButton(ButtonEvent<String>),
	GameButton(ButtonEvent<GameButton>),
	Button(ButtonEvent<ReplayButton>),
}

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	pub solution: Solution,
}

pub struct EditInterface {
	replay: Replay,
	input: NodeRef,
	button_handler: ButtonHandler<ReplayButton>,
}

impl Component for EditInterface {
	type Message = Msg;
	type Properties = Props;

	fn create(ctx: &Context<Self>) -> Self {
		let props = ctx.props();
		Self {
			replay: Replay::from_solution(props.solution),
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
	      		<GameComponent puzzle={self.replay.get_puzzle().clone()}
					top_left={{ html! { <>
						<ButtonComponent
							onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| ReplayButton::Play)))}>
							<img src="/assets/play.png"
								alt="Enter Play Mode"/>
						</ButtonComponent>
					</> }}}
	      			top_right={{ html! { <>
		        		<ButtonComponent
		        			onbutton={ctx.link().callback(
								|e: ButtonEvent<()>| Msg::Button(
									e.map(|_| ReplayButton::Edit)))}>
		        			<img src="/assets/edit.png"
		        				alt="Edit Puzzle"/>
		        		</ButtonComponent>
	      			</> }}}/>
				<div class="row">
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::GoToStart)))}>
						<img src="/assets/go_to_start.png"
							alt="Go To Start"/>
					</ButtonComponent>
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::Back)))}>
						<img src="/assets/back.png"
							alt="Backward"/>
					</ButtonComponent>
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::Forward)))}>
						<img src="/assets/forward.png"
							alt="Forward"/>
					</ButtonComponent>
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::GoToEnd)))}>
						<img src="/assets/go_to_end.png"
							alt="Go To End"/>
					</ButtonComponent>
				</div>
				<div class="thin-row">
					<h3>{"Share Replay"}</h3>
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::Import)))}>
						<img src="/assets/import.png"
							alt="Import"/>
					</ButtonComponent>
					<ButtonComponent
						onbutton={ctx.link().callback(
						move |e: ButtonEvent<()>|
							Msg::Button(e.map(|_|
								ReplayButton::Export)))}>
						<img src="/assets/export.png"
							alt="Export"/>	
						<div class="copied">
							<p>{"Copied link!"}</p>
						</div>
					</ButtonComponent>
	      		<div class="row">
	      			<input type="text"
	      				ref={self.input.clone()}
	      				placeholder="https://ztrix-game.web.app/game/..."
	      				onkeydown={Callback::from(
	      					|e: KeyboardEvent|
	      						e.stop_propagation())}/>
	      		</div>
	        </KeyboardInterface>
	    }
	}

	fn update(&mut self, ctx: &Context<Self>,
			msg: Self::Message) -> bool {
		// let user_prefs = UserPrefs::get();
    	// let key_bindings = &user_prefs.key_bindings;
    	// let event = match msg {
    	// 	Msg::KeyButton(event) =>
    	// 		match event.maybe_map(|b|
    	// 			key_bindings.edit_bindings.get(&b)
    	// 			.copied())
    	// 			.and_then(|e| self.button_handler
    	// 				.update(e)) {
		// 			Some(event) => event,
		// 			None => return false,
		// 		}
    	// 	Msg::Button(event) => match self.button_handler
    	// 			.update(event) {
		// 			Some(event) => event,
		// 			None => return false,
    	// 		}
		// 	Msg::Button(event) => event,
		// };
		// match event {
		// 	ButtonEvent::Press(b) => match b {
		// 		ReplayButton::SetHold => {
		// 			let hold = &mut self.puzzle.game.hold;
		// 			*hold = match *hold {
		// 				Some(PieceType::T) => None,
		// 				Some(p) => Some(cycle_piece(p)),
		// 				None => Some(PieceType::I),
		// 			};
		// 			update_bag(&mut self.puzzle.game, 0);
		// 		},
		// 		ReplayButton::SetCurrent => {
		// 			let piece = &mut self.puzzle.game.piece;
		// 			*piece = match piece {
		// 				Some(p) => match p.get_type() {
		// 					PieceType::T => None,
		// 					p => Some(MaybeActive::Inactive(
		// 						cycle_piece(p))),
		// 				},
		// 				None => Some(MaybeActive::Inactive(
		// 					PieceType::I)),
		// 			};
		// 			update_bag(&mut self.puzzle.game, 0);
		// 		},
		// 		ReplayButton::SetNext(n) => {
		// 			let queue = &mut self.puzzle.game.queue;
		// 			queue[n-1] = cycle_piece(queue[n-1]);
		// 			update_bag(&mut self.puzzle.game, 0);
		// 		},
		// 		ReplayButton::SetBagPos => {
		// 			update_bag(&mut self.puzzle.game, 1);
		// 		},
		// 		ReplayButton::ToggleHoldUsed => {
		// 			self.puzzle.game.has_held = !self.puzzle.game.has_held;
		// 		},
		// 		ReplayButton::ToggleZone => {
		// 			self.puzzle.game.in_zone = !self.puzzle.game.in_zone;
		// 		},
		// 		ReplayButton::Import => {
		// 			let input = self.input
		// 				.cast::<HtmlInputElement>()
		//     			.expect("element should be an input");
	    // 			let value = input.value();
	    // 			let prefix1 = vec![
	    // 				"https://", "http://", ""];
	    // 			let prefix2 = vec![
	    // 				"ztrix-game.web.app/",
	    // 				"152.7.71.114/",
	    // 				"localhost/", "/", ""];
	    // 			let prefix3 = vec![
	    // 				"game/", "play/", "edit/", "puzzle/", ""];
	    // 			let mut code = None;
	    // 			for p1 in prefix1.iter() {
	    // 				for p2 in prefix2.iter() {
	    // 					for p3 in prefix3.iter() {
	    // 						let prefix = format!{
	    // 							"{}{}{}", p1, p2, p3};
	    // 						code = code.or_else(
	    // 							|| value.strip_prefix(&prefix));
	    // 					}
	    // 				}
	    // 			}
	    // 			match code.and_then(|c|
	    // 				Puzzle::from_str(c).ok()) {
	    // 				Some(puzzle) => {
	    // 					self.puzzle = puzzle;
	    // 				}
	    // 				None => match code.and_then(|c|
		// 					Game::from_str(c).ok()) {
		// 					Some(game) => {
		// 						self.puzzle = Puzzle::new(game);
		// 					}
		// 					None => (),
		// 				}
	    // 			}
		// 		},
		// 		ReplayButton::Export => {
		// 			let input = self.input
		// 				.cast::<HtmlInputElement>()
		//     			.expect("element should be an input");
		//     		let value = format!{
		//     			"https://ztrix-game.web.app/puzzle/{}", self.puzzle};
		//     		input.set_value(&value);
		// 			let window = web_sys::window()
		// 				.expect("should be a window");
		// 			let navigator = window.navigator();
		// 			if let Some(clipboard) = navigator.clipboard() {
		// 				let _ = clipboard.write_text(&value);
		// 			}
		// 		},
		// 		ReplayButton::Revert =>
		// 			self.puzzle = self.initial.clone(),
		// 		ReplayButton::EraseAll =>
		// 			self.puzzle = Puzzle::default(),
		// 		_ => (),
		// 	}
		// 	ButtonEvent::Release(b) => match b {
		// 		ReplayButton::SetQueue => {
		// 			let queue = &mut self.puzzle.game.queue;
		// 			let start_fill = queue.fill();
		// 			let string = queue.pieces.iter()
		// 				.map(|p| p.serialize())
		// 				.collect::<Vec<String>>().join("");
		// 			let string = web_sys::window()
		// 				.expect("should be a window")
		// 				.prompt_with_message_and_default(
		// 					"Set Queue: ", &string)
		// 				.unwrap_or(None).unwrap_or(string);
		// 			queue.pieces = VecDeque::new();
		// 			let mut input = DeserializeInput::from(&string);
		// 			while let Ok(p) = PieceType::deserialize(&mut input) {
		// 				queue.pieces.push_back(p);
		// 			}
		// 			let end_fill = queue.fill();
		// 			let advance = if start_fill > end_fill {
		// 				(start_fill - end_fill) % 7
		// 			} else {
		// 				7 - (end_fill - start_fill) % 7
		// 			};
		// 			update_bag(&mut self.puzzle.game, advance);
		// 		}		
		// 		ReplayButton::Play => {
		// 			let history = ctx.link().history()
		// 				.expect("should be a history");
		// 			history.replace(
		// 				Route::EditPuzzle {
		// 					puzzle: self.puzzle.clone()
		// 				}
		// 			);
		// 			history.push(
		// 				Route::PuzzlePuzzle {
		// 					puzzle: self.puzzle.clone()
		// 				}
		// 			)
		// 		}
		// 		_ => (),
		// 	}
		// }
		true
	}
}