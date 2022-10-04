
use component::keyboard_interface::KeyboardInterface;
use component::button::ButtonComponent;
use component::game::GameButton;
use component::queue::QueueButton;
use controller::input_handler::ButtonEvent;

use controller::input_handler::InputHandler;

use controller::action_handler::MetaAction;
use controller::action_handler::ActionHandler;

use component::game::GameComponent;



use yew::prelude::*;

use ztrix::game::Game;
use ztrix::game::Mino;
use ztrix::replay::Replay;

use user_prefs::UserPrefs;

use gloo_timers::callback::Interval;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum PlayButton {
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
    RerollCurrent,
    RerollNext(usize),
	Restart,
}

impl PlayButton {
	pub fn get_name(&self) -> String {
        match self {
            PlayButton::Left => "Move Left",
            PlayButton::Right => "Move Right",
            PlayButton::DownSlow => "Move Down",
            PlayButton::DownFast => "Instant Drop",
            PlayButton::Clockwise => "Rotate CW",
            PlayButton::Anticlockwise => "Rotate ACW",
            PlayButton::Flip => "Rotate 180",
            PlayButton::Place => "Place Piece",
            PlayButton::Hold => "Hold Piece",
            PlayButton::Zone => "Toggle Zone",
            PlayButton::Undo => "Undo",
            PlayButton::Redo => "Redo",
            PlayButton::RerollCurrent => "Reroll Current",
            PlayButton::RerollNext(n) =>
            	return format!{"Reroll Next #{}", n},
            PlayButton::Restart => "Restart",
        }.to_string()
    }

    pub fn get_icon_url(&self) -> Option<String> {
        match self {
            PlayButton::Left => Some("/assets/left.png"),
            PlayButton::Right => Some("/assets/right.png"),
            PlayButton::DownSlow => Some("/assets/down.png"),
            PlayButton::DownFast => Some("/assets/instant.png"),
            PlayButton::Clockwise => Some("/assets/cw.png"),
            PlayButton::Anticlockwise => Some("/assets/anticw.png"),
            PlayButton::Flip => Some("/assets/180.png"),
            PlayButton::Place => Some("/assets/place.png"),
            PlayButton::Hold => Some("/assets/hold.png"),
            PlayButton::Zone => Some("/assets/zone.png"),
            PlayButton::Undo => Some("/assets/undo.png"),
            PlayButton::Redo => Some("/assets/redo.png"),
            PlayButton::RerollCurrent => Some("/assets/reroll.png"),
            PlayButton::Restart => Some("/assets/restart.png"),
            _ => None,
        }.and_then(|s| Some(s.to_string()))
    }

    pub fn view_button(self, ctx: &Context<PlayInterface>)
    		-> Html {
    	html! {
			<ButtonComponent
				onbutton={ctx.link().callback(
					move |u: ButtonEvent<()>| Msg::UserButton(
						u.map(|_| self)))}>
				{match self.get_icon_url() {
					None => html! {
						<p>{self.get_name()}</p>
					},
					Some(src) => html! {
						<img src={src}
							alt={self.get_name()}/>
					},
				}}
			</ButtonComponent>
		}
    }
}

pub enum Msg {
	KeyButton(ButtonEvent<String>),
	GameButton(ButtonEvent<GameButton>),
	UserButton(ButtonEvent<PlayButton>),
	Interval,
	Edit,
}

#[derive(Properties, PartialEq)]
#[derive(Default)]
pub struct Props {
	#[prop_or_default]
	pub game: Game,
}

pub struct PlayInterface {
	replay: Replay,
	input_handler: InputHandler<PlayButton>,
	action_handler: ActionHandler,
	_interval: Interval,
}

impl Component for PlayInterface {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
		let link = ctx.link().clone();
        Self {
        	replay: Replay::new(ctx.props().game.clone()),
        	input_handler: InputHandler::new(),
        	action_handler: ActionHandler::new(),
        	_interval: Interval::new(16, move ||
				link.send_message(Msg::Interval))
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
    	let user_prefs = UserPrefs::get();
     	let input_bindings = user_prefs.get_input_bindings();
        html! {
        	<KeyboardInterface
        		onkey={ctx.link().callback(
        			|e: ButtonEvent<String>|
        				Msg::KeyButton(e))}>
            	<GameComponent
            		game={self.replay.get_game().clone()}
	      			top_right={{ html! {
		        		<ButtonComponent
		        			onbutton={ctx.link().batch_callback(
								|b: ButtonEvent<()>| match b {
									ButtonEvent::Press(_) =>
										Some(Msg::Edit),
									_ => None,
								})}>
		        			<p>{"Edit"}</p>
		        		</ButtonComponent>
	      				}}}
		        	bottom_left={{html! { <>
		        		<p><strong>{"FRAME"}</strong></p>
		        		<p><strong>
		        			{self.replay.get_frame()}
		        		</strong></p>
		        		{input_bindings
			        		.get_left_buttons()
			            	.iter().map(|b| {
		        				b.view_button(ctx)
		    				}).collect::<Html>()}
		            </> }}}
		        	bottom_right={input_bindings
		        		.get_right_buttons()
		            	.iter().map(|b| {
	        				b.view_button(ctx)
	    				}).collect::<Html>()}
	    			onbutton={ctx.link().callback(
	        			|e: ButtonEvent<GameButton>|
	        				Msg::GameButton(e))}/>
	            {for input_bindings.get_bottom_buttons()
	            	.iter().map(|v| html! {
	            		<div class="row">
	            			{for v.iter().map(|b| {
	            				b.view_button(ctx)
            				})}
	     				</div>
	            	})}
            </KeyboardInterface>
        }
    }

       fn update(&mut self, _ctx: &Context<Self>,
    		msg: Msg) -> bool {
    	let user_prefs = UserPrefs::get();
    	let input_bindings = user_prefs.get_input_bindings();
    	let event = match msg {
    		Msg::KeyButton(event) => {
    			match event.maybe_map(|b|
    				input_bindings.map_key_to_play(&b))
    				.and_then(|e| self.input_handler
    					.button_event(e)) {
					Some(event) => event,
					None => return false,
				}
    		}
    		Msg::GameButton(event) => {
    			match event.maybe_map(|b| match b {
    				GameButton::Queue(
    					QueueButton::NextBox(n)) =>
    						Some(PlayButton::RerollNext(n)),
    				_ => None,
    			}).and_then(|e| self.input_handler
    				.button_event(e)) {
					Some(event) => event,
					None => return false,
    			}
    		}
    		Msg::UserButton(event) => match self.input_handler
    				.button_event(event) {
					Some(event) => event,
					None => return false,
    			}
    		Msg::Interval => {
    			self.input_handler.time_passed()
    		}
    		Msg::Edit => {
				let mut game = self.replay.get_game().clone();
				for row in game.board.matrix.iter_mut() {
					for mino in row.iter_mut() {
						*mino = match mino {
							None => None,
							Some(_) => Some(Mino::Gray),
						}
					}
				}
				let window = web_sys::window()
					.expect("should be a window");
				let url = format!{"/edit/{}", game};
				window.open_with_url_and_target(
					&url, "_blank")
					.expect("should be able to open url");
				return true;
    		}
    	};
		let meta_actions = self.action_handler
			.update(&self.replay, event);
    	meta_actions.into_iter().map(|e| match e {
    			MetaAction::Action(action) =>
    				self.replay.update(action),
    			MetaAction::Revert => self.replay.revert(),
    			MetaAction::Undo => self.replay.undo(),
    			MetaAction::Redo => self.replay.redo(),
    			MetaAction::Reroll(back) =>
    				self.replay.reroll_backward(back),
    			MetaAction::Restart => {
    				for _ in 0..self.replay.get_frame() {
    					self.replay.undo();
    				}
				},
    		}).count() != 0
    }
}
